#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

typedef vector<char> pile;

// reflects which point of the file
enum status
{
    DRAWING = 0,
    MOVING = 1
};

int readDrawing(string line, vector<pile> &stacks)
{
    // each crate is 4 char wide [X]*space*
    int line_size = line.size();
    for (int i = 0; i < line_size; i += 4)
    {
        int stack = i / 4;
        // value of each crate is stored in first index of each 4 char
        char crate = line[i + 1];
        if (crate == ' ')
        {
            continue;
        }
        // finding a digit means were at the end of the drawing
        else if (isdigit(crate))
        {
            return status::MOVING;
        }
        stacks[stack].insert(stacks[stack].begin(), crate);
    }
    return status::DRAWING;
}

void moveCrates(string line, vector<pile> &stacks)
{
    int quantity, fromStack, toStack;
    sscanf(line.c_str(), "move %d from %d to %d", &quantity, &fromStack, &toStack);
    fromStack--;
    toStack--;
    pile temp;
    for (int i = 0; i < quantity; i++)
    {
        temp.push_back(stacks[fromStack].back());
        stacks[fromStack].pop_back();
    }

    reverse(temp.begin(), temp.end());
    for (int i = 0; i < temp.size(); i++)
    {
        stacks[toStack].push_back(temp[i]);
    }
}

int main()
{
    ifstream file;
    file.open("input.txt");
    if (file.is_open())
    {
        string line;
        int status = status::DRAWING;
        // get a early line to define the number of stacks there are
        getline(file, line);
        int totalStacks = (line.length() + 1) / 4;
        vector<pile> stacks(totalStacks);

        // do while instead of while because the first line needs to be used aswell
        do
        {
            // skip blank line beetween the drawing and movements
            if (line.empty())
            {
                continue;
            }
            if (status == status::DRAWING)
            {
                status = readDrawing(line, stacks);
                continue;
            }
            moveCrates(line, stacks);
        } while (getline(file, line));

        for (pile pile : stacks)
        {
            cout << pile.back();
        }
    }
    file.close();
    return 0;
}