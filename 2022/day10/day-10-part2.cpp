#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <set>

using namespace std;

typedef vector<char> Line;
typedef vector<Line> CRT;

void parserInstrctions(string line, string &instruction, int &value)
{
    instruction = line.substr(0, line.find(' '));
    if (instruction == "addx")
    {
        value = stoi(line.substr(line.find(' ')));
    }
}

bool isCheckPoint(int &cycleCounter)
{
    if (cycleCounter % 40 == 0)
    {
        cycleCounter = 0;
        return true;
    }
    return false;
}

char drawPixel(int counter, vector<int> sprite)
{
    if (find(sprite.begin(), sprite.end(), counter) != sprite.end())
    {
        return '#';
    }
    return '.';
}

void updateCrt(int spriteMiddle, int &cycleCounter, Line &crtLine)
{
    // create a vector with sprite locations
    vector<int> vec = {spriteMiddle, spriteMiddle - 1, spriteMiddle + 1};
    crtLine.push_back(drawPixel(cycleCounter, vec));
    cycleCounter++;
}

int main()
{
    ifstream file;
    file.open("input.txt");
    if (file.is_open())
    {
        string line;
        int value;
        string instruction;
        // starting values
        int cycleCounter = 0;
        int X = 1;
        CRT crt;
        Line crtLine;
        while (getline(file, line))
        {
            parserInstrctions(line, instruction, value);
            // case the instruction is noop
            if (instruction == "noop")
            {
                if (isCheckPoint(cycleCounter))
                {
                    crt.push_back(crtLine);
                    crtLine.clear();
                }
                updateCrt(X, cycleCounter, crtLine);
                continue;
            }
            // case the instruction is addx
            for (int i = 0; i < 2; i++)
            {
                if (isCheckPoint(cycleCounter))
                {
                    crt.push_back(crtLine);
                    crtLine.clear();
                }
                updateCrt(X, cycleCounter, crtLine);
            }
            // update register
            X += value;
        }
        // pushes the last line to the crt
        crt.push_back(crtLine);
        // print crt
        for (Line l : crt)
        {
            for (char c : l)
            {
                cout << c;
            }
            cout << endl;
        }
    }
    file.close();
    return 0;
}