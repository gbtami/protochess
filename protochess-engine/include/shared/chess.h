#pragma once

#include <string>
#include <boost/dynamic_bitset.hpp>
#include "../../src/types.h"
#include "../../src/player.h"
#include "../../src/Board.h"

class Chess {
private:
    Board board;
    std::map<int, Player> players;
    int playerCounter = 0;
    int whosTurn = 0;
    Dimensions dimensions;

    std::map<boost::uuids::uuid, Piece> charToPieceBitsets(const char *pieces);

    std::map<char, MovementPattern> charToKnownMP(const char *pieces);

public:
    Chess();

    Chess(int width, int height);

    int registerPlayer(std::string playerName);

    void buildClassicSet();

    std::string toString();

    void reset();
};

