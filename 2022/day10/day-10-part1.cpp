#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <set>

using namespace std;

void parserInstrctions(string line, string &instruction, int &value)
{
    instruction = line.substr(0, line.find(' '));
    if (instruction == "addx")
    {
        value = stoi(line.substr(line.find(' ')));
    }
}

bool isCheckPoint(int cycleCounter)
{
    if ((cycleCounter - 20) % 40 == 0 || cycleCounter == 20)
    {
        return true;
    }
    return false;
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
        vector<int> tracker;
        while (getline(file, line))
        {
            parserInstrctions(line, instruction, value);
            // case the instruction is noop
            if (instruction == "noop")
            {
                cycleCounter++;
                if (isCheckPoint(cycleCounter))
                {
                    tracker.push_back(cycleCounter * X);
                }
                continue;
            }
            // case the instruction is addx
            for (int i = 0; i < 2; i++)
            {
                cycleCounter++;
                if (isCheckPoint(cycleCounter))
                {
                    tracker.push_back(X * cycleCounter);
                }
            }
            X += value;
        }
        // sum the result
        int sum = 0;
        for (int i : tracker)
        {
            sum += i;
        }
        cout << sum;
    }
    file.close();
    return 0;
}