use std::collections::{HashMap};
use dependencies::specs::{System, RunArg};
use event::{FrontChannel, BackChannel};
use event_enums::ai_x_control::{AiToControl, AiFromControl};
use event_enums::main_x_control::{MainToControl, MainFromControl};
use event_enums::control_x_player::{ControlToPlayer, ControlFromPlayer};
use utils::{Delta, Player};

#[derive(Debug)]
pub struct ControlSystem {
    main_back_channel: BackChannel<MainToControl, MainFromControl>,
    ai_back_channel: BackChannel<AiToControl, AiFromControl>,
    player_front_channel: Option<FrontChannel<ControlToPlayer, ControlFromPlayer>>,
    repeat_map: HashMap<RepeatEvent, ControlToPlayer>,
    time: f64,
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum RepeatEvent {
    Up(Player),
    Down(Player),
    Left(Player),
    Right(Player),
    Joy(Player),
}

impl ControlSystem {
    pub fn new(
        main_back_channel: BackChannel<MainToControl, MainFromControl>,
        ai_back_channel: BackChannel<AiToControl, AiFromControl>,
        player_front_channel: FrontChannel<ControlToPlayer, ControlFromPlayer>,
    ) -> ControlSystem {
        ControlSystem {
            main_back_channel: main_back_channel,
            ai_back_channel: ai_back_channel,
            player_front_channel: Some(player_front_channel),
            repeat_map: HashMap::new(),
            time: 0.0,
        }
    }

    fn process_main_event(&mut self, event: MainToControl) {
        match event {
            MainToControl::Up(amount, player) => self.send_repeat(ControlToPlayer::Up(amount, player)),
            MainToControl::Down(amount, player) => self.send_repeat(ControlToPlayer::Down(amount, player)),
            MainToControl::Left(amount, player) => self.send_repeat(ControlToPlayer::Left(amount, player)),
            MainToControl::Right(amount, player) => self.send_repeat(ControlToPlayer::Right(amount, player)),
            MainToControl::Joy(x, y, player) => self.send_repeat(ControlToPlayer::Joy(x, y, player)),
        }
    }

    fn process_ai_event(&mut self, event: AiToControl) {
        match event {
            AiToControl::Up(amount, player) => self.send_once(ControlToPlayer::Up(amount, player)),
            AiToControl::Down(amount, player) => self.send_once(ControlToPlayer::Down(amount, player)),
            AiToControl::Left(amount, player) => self.send_once(ControlToPlayer::Left(amount, player)),
            AiToControl::Right(amount, player) => self.send_once(ControlToPlayer::Right(amount, player)),
            AiToControl::Joy(x, y, player) => self.send_once(ControlToPlayer::Joy(x, y, player)),
        }
    }

    fn send_repeat(&mut self, event: ControlToPlayer) {
        match &event {
            &ControlToPlayer::Up(_, player) => self.repeat_map.insert(RepeatEvent::Up(player), event),
            &ControlToPlayer::Down(_, player) => self.repeat_map.insert(RepeatEvent::Down(player), event),
            &ControlToPlayer::Right(_, player) => self.repeat_map.insert(RepeatEvent::Right(player), event),
            &ControlToPlayer::Left(_, player) => self.repeat_map.insert(RepeatEvent::Left(player), event),
            &ControlToPlayer::Joy(_, _, player) => self.repeat_map.insert(RepeatEvent::Joy(player), event),
        };
    }

    fn send_once(&mut self, event: ControlToPlayer) {
        self.player_front_channel.as_mut().unwrap_or_else(|| panic!("Player Front Channel was none")).send_to(event);
    }

    fn trigger_repeats(&mut self) {
        let mut channel = self.player_front_channel.take().unwrap_or_else(|| panic!("Player Front Channel was none"));
        for value in self.repeat_map.values() {
            channel.send_to(value.clone());
        }
        self.player_front_channel = Some(channel);
    }
}

impl System<Delta> for ControlSystem {
    fn run(&mut self, arg: RunArg, delta_time: Delta) {
        self.time += delta_time;

        if self.time >= 300.0 {
            self.time = 0.0;
            self.main_back_channel.send_from(MainFromControl::Save);
        }

        let mut needs_fetch = (true, true);

        while needs_fetch.0 || needs_fetch.1 {
            if let Some(event) = self.main_back_channel.try_recv_to() {
                self.process_main_event(event);
            } else {
                needs_fetch.0 = false;
            }

            if let Some(event) = self.ai_back_channel.try_recv_to() {
                self.process_ai_event(event);
            } else {
                needs_fetch.1 = false;
            }
        }

        self.trigger_repeats();

        arg.fetch(|_| ());
    }
}
