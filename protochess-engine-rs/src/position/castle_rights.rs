#[derive(Clone)]
pub struct CastleRights(u8, u8);

//Castling rights for up to 8 players
//CastleRights.0 -- kingside rights
//CastleRights.1 -- Queenside rights
//Where each bit in the u8 represents the castling right for the player at that index
//Ex if CastleRights.0 == 1u8 then the 0th player can castle kingside
impl CastleRights {
    pub fn new() -> CastleRights {
        CastleRights(255u8, 255u8)
    }

    pub fn can_player_castle_kingside(&self, playernum: u8) -> bool {
        (self.0 >> playernum) & 1u8 != 0
    }

    pub fn can_player_castle_queenside(&self, playernum: u8) -> bool {
        (self.1 >> playernum) & 1u8 != 0
    }

    pub fn can_player_castle(&self, playernum:u8) -> bool {
        self.can_player_castle_kingside(playernum) ||
            self.can_player_castle_queenside(playernum)
    }

    pub fn disable_kingside_castle(&mut self, playernum: u8) {
        self.0 |= 1u8 << playernum
    }

    pub fn disable_queenside_castle(&mut self, playernum: u8) {
        self.1 |= 1u8 << playernum
    }
}
