impl TarockState {
    fn nextMoves(&self) -> Vec<TarockState> {
        switch self.game.state {
            TarockGameState::Bidding => genBiddingMoves(self),
            TarockGameState::Playing => genPlayingMoves(self),
            TarockGameState::Solorufer => genSoloruferMoves(self),
            TarockGameState::End => Vec::new(),
        }
    }
}

fn genSoloruferMoves(state: &TarockState) -> Vec<TarockState> {
    let mut moves = Vec::new();
    let possibleCards = state.cards.players[state.game.currentPlayer].hand.cards;
    for card in possibleCards {
        card
    }
    moves
}