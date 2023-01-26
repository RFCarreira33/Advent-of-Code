#include <iostream>
#include <fstream>
#include <map>
#include <vector>

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

const map<char, results> endings = {
    {'X', results::LOSE},
    {'Y', results::DRAW},
    {'Z', results::WIN},
};

// first value of pair wins second loses
const map<plays, pair<plays, plays>> counterPlays = {
    {plays::ROCK, make_pair(plays::PAPER, plays::SCISSORS)},
    {plays::PAPER, make_pair(plays::SCISSORS, plays::ROCK)},
    {plays::SCISSORS, make_pair(plays::ROCK, plays::PAPER)},
};

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
            // store the oponents choice and match ending
            plays oponent = opponentPlays.at(line[0]);
            int ending = (int)endings.at(line[2]);
            if (ending == results::DRAW)
            {
                score += ending + (int)oponent;
                continue;
            }
            // if it not a draw pick from counterPlays
            // get the pair of plays for the oponent pick
            pair<plays, plays> counters = counterPlays.at(oponent);
            switch (ending)
            {
            case results::WIN:
                score += ending + (int)counters.first;
                break;

            case results::LOSE:
                score += (int)counters.second;
                break;
            }
        }
    }
    file.close();
    cout << "Total score is " << score;

    return 0;
}