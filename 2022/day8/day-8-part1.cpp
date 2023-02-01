#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <map>

using namespace std;

typedef vector<int> Row;
typedef vector<Row> Forest;

// spaguetti
// there is probably a better way to do this but I was tilted
int scanDirections(Forest forest, int &row, int &col)
{
    int highestTree = -1;
    int tree = forest[row][col];
    int size = forest.size();
    // check below
    for (int i = row + 1; i < size; i++)
    {
        if (forest[i][col] > highestTree)
        {
            highestTree = forest[i][col];
        }
    }
    if (tree > highestTree)
    {
        return 1;
    }
    // check above
    highestTree = -1;
    for (int i = row - 1; i >= 0; i--)
    {
        if (forest[i][col] > highestTree)
        {
            highestTree = forest[i][col];
        }
    }
    if (tree > highestTree)
    {
        return 1;
    }
    // check right
    highestTree = -1;
    for (int i = col + 1; i < size; i++)
    {
        if (forest[row][i] > highestTree)
        {
            highestTree = forest[row][i];
        }
    }
    if (tree > highestTree)
    {
        return 1;
    }
    // check left
    highestTree = -1;
    for (int i = col - 1; i >= 0; i--)
    {
        if (forest[row][i] > highestTree)
        {
            highestTree = forest[row][i];
        }
    }
    if (tree > highestTree)
    {
        return 1;
    }
    return 0;
}

int main()
{
    ifstream file;
    file.open("input.txt");
    if (file.is_open())
    {
        Forest forest;
        string line;

        while (getline(file, line))
        {
            Row row;
            for (char c : line)
            {
                int tree = c - '0';
                row.push_back(tree);
            }
            forest.push_back(row);
        }
        int size = forest.size();
        // add the borders to the counter
        int counter = size * 4 - 4;
        for (int tempRow = 1; tempRow < size - 1; tempRow++)
        {
            for (int tempCol = 1; tempCol < size - 1; tempCol++)
            {
                counter += scanDirections(forest, tempRow, tempCol);
            }
        }
        cout << counter;
    }
    file.close();
    return 0;
}
