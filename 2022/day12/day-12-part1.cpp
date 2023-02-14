#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <queue>

using namespace std;

// changes in direction to check around the cell
const int dx[] = {-1, 0, 1, 0};
const int dy[] = {0, 1, 0, -1};

typedef vector<vector<char>> Mountain;

struct Coordenates
{
    int x;
    int y;
};

Mountain parseMatrix(ifstream &file)
{
    string line;
    Mountain mountain;
    vector<char> temp;
    while (getline(file, line))
    {
        for (char c : line)
        {
            temp.push_back(c);
        }
        mountain.push_back(temp);
        temp.clear();
    }
    return mountain;
}

pair<Coordenates, Coordenates> getStartEnd(Mountain &mountain)
{
    Coordenates start;
    Coordenates end;
    for (int i = 0; i < mountain.size(); i++)
    {
        for (int j = 0; j < mountain[0].size(); j++)
        {
            if (mountain[i][j] == 'S')
            {
                start = {i, j};
                mountain[i][j] = 'a';
            }
            else if (mountain[i][j] == 'E')
            {
                end = {i, j};
                mountain[i][j] = 'z';
            }
        }
    }
    return pair<Coordenates, Coordenates>{start, end};
}

vector<Coordenates> getNeighbours(Coordenates current)
{
    vector<Coordenates> vec;
    for (int i = 0; i < 4; i++)
    {
        vec.push_back({current.x - dx[i], current.y - dy[i]});
    }
    return vec;
}

// https://en.wikipedia.org/wiki/Breadth-first_search
int BFS(Mountain mountain, Coordenates start, Coordenates end)
{
    // input sizes
    int m = mountain.size(), n = mountain[0].size();
    // explored should mimic matrix size
    vector<vector<bool>> explored(m, vector<bool>(n, false));
    explored[start.x][start.y] = true;
    queue<pair<Coordenates, int>> q;
    q.push({start, 0});
    while (!q.empty())
    {
        Coordenates coord = q.front().first;
        int steps = q.front().second;
        if (coord.x == end.x && coord.y == end.y)
        {
            return steps;
        }
        q.pop();
        char from = mountain[coord.x][coord.y];
        vector<Coordenates> neighbours = getNeighbours(coord);
        for (Coordenates c : neighbours)
        {
            if (c.x < 0 || c.y < 0 || c.x >= m || c.y >= n)
            {
                continue;
            }
            char to = mountain[c.x][c.y];
            if (to - from > 1 || explored[c.x][c.y])
            {
                continue;
            }
            explored[c.x][c.y] = true;
            q.push({c, steps + 1});
        }
    }
    return -1;
}

int main()
{
    ifstream file;
    file.open("input.txt");
    Mountain map;
    pair<Coordenates, Coordenates> startEnd;
    if (file.is_open())
    {
        map = parseMatrix(file);
        startEnd = getStartEnd(map);
        cout << BFS(map, startEnd.first, startEnd.second);
    }
    file.close();
    return 0;
}