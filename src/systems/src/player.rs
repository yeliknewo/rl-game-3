use components::{CompPlayer, CompMoving};
use dependencies::specs::{System, RunArg, Join};
use event::{BackChannel};
use event_enums::control_x_player::{ControlToPlayer, ControlFromPlayer};
use utils::{Delta, Coord};

pub struct PlayerSystem {
    control_back_channel: BackChannel<ControlToPlayer, ControlFromPlayer>,
}

impl PlayerSystem {
    pub fn new(
        control_back_channel: BackChannel<ControlToPlayer, ControlFromPlayer>
    ) -> PlayerSystem {
        PlayerSystem {
            control_back_channel: control_back_channel,
        }
    }
}

impl System<Delta> for PlayerSystem {
    fn run(&mut self, arg: RunArg, _: Delta) {
        let (players, mut movings) = arg.fetch(|w| (
            w.read::<CompPlayer>(),
            w.write::<CompMoving>(),
        ));

        while let Some(event) = self.control_back_channel.try_recv_to() {
            match event {
                ControlToPlayer::Right(amount, player_evt) => {
                    for(player, mut moving) in (&players, &mut movings).iter() {
                        if player.get_player() == player_evt {
                            moving.get_mut_velocity().x += amount as Coord;
                        }
                    }
                },
                ControlToPlayer::Left(amount, player_evt) => {
                    for(player, mut moving) in (&players, &mut movings).iter() {
                        if player.get_player() == player_evt {
                            moving.get_mut_velocity().x -= amount as Coord;
                        }
                    }
                },
                ControlToPlayer::Up(amount, player_evt) => {
                    for(player, mut moving) in (&players, &mut movings).iter() {
                        if player.get_player() == player_evt {
                            moving.get_mut_velocity().y += amount as Coord;
                        }
                    }
                },
                ControlToPlayer::Down(amount, player_evt) => {
                    for(player, mut moving) in (&players, &mut movings).iter() {
                        if player.get_player() == player_evt {
                            moving.get_mut_velocity().y -= amount as Coord;
                        }
                    }
                },
            }
        }
    }
}
