#include <iostream>
#include <fstream>
#include <map>

using namespace std;

enum results
{
    WIN = 6,
    DRAW = 3,
    LOSE = 0
};

enum plays
{
    ROCK = 1,
    PAPER = 2,
    SCISSORS = 3
};

const map<char, plays> opponentPlays = {
    {'A', plays::ROCK},
    {'B', plays::PAPER},
    {'C', plays::SCISSORS},
};

const map<char, plays> playerPlays = {
    {'X', plays::ROCK},
    {'Y', plays::PAPER},
    {'Z', plays::SCISSORS},
};

// subtracting oponent plays from player
// if the result is -1 or 2 oponents loses
// if the result is 1 or -2 player loses
// if 0 draw

int main()
{
    ifstream file;
    file.open("input.txt");
    int score = 0;
    if (file.is_open())
    {
        string line;
        while (getline(file, line))
        {
            // store the value of the players choices
            int oponent = (int)opponentPlays.at(line[0]);
            int player = (int)playerPlays.at(line[2]);
            int match = oponent - player;

            if (match == 0)
            {
                score += results::DRAW + player;
                continue;
            }
            if (match == 2 || match == -1)
            {
                score += results::WIN + player;
                continue;
            }
            score += player;
        }
    }
    file.close();
    cout << "Total score is " << score;

    return 0;
}