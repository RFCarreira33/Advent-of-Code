#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <set>

using namespace std;

typedef pair<int, int> Coordenates;

Coordenates handleMovement(Coordenates &head, char direction)
{
    switch (direction)
    {
    case 'U':
        head.second++;
        break;

    case 'D':
        head.second--;
        break;

    case 'L':
        head.first--;
        break;

    case 'R':
        head.first++;
        break;
    }
    return head;
}

// if any of the parts of the tail are 2 or -2 indexes away from head
bool isTailFar(Coordenates head, Coordenates tail)
{
    if (abs(head.first - tail.first) > 1 || abs(head.second - tail.second) > 1)
    {
        return true;
    }
    return false;
}

// return the way the tail should be updated for each part
int updateTail(int head, int tail)
{
    if (head - tail > 0)
    {
        return 1;
    }
    if (head - tail < 0)
    {
        return -1;
    }
    return 0;
}

int main()
{
    ifstream file;
    file.open("input.txt");
    vector<Coordenates> LocationsTracked;
    if (file.is_open())
    {
        Coordenates head = {0, 0};
        Coordenates tail = {0, 0};
        set<Coordenates> LocationsTracked = {{0, 0}};
        string line;
        char direction;
        int steps;
        while (getline(file, line))
        {
            sscanf(line.c_str(), "%c %d", &direction, &steps);
            for (int i = 0; i < steps; i++)
            {
                handleMovement(head, direction);
                if (isTailFar(head, tail))
                {
                    tail.first += updateTail(head.first, tail.first);
                    tail.second += updateTail(head.second, tail.second);
                }
                LocationsTracked.insert(tail);
            }
        }
        cout << LocationsTracked.size();
    }
    file.close();
    return 0;
}