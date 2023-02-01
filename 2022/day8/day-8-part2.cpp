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
    int tree = forest[row][col];
    int size = forest.size();
    // check below
    int bottom = 0;
    for (int i = row + 1; i < size; i++)
    {
        bottom++;
        if (tree <= forest[i][col])
        {
            break;
        }
    }
    // check above
    int top = 0;
    for (int i = row - 1; i >= 0; i--)
    {
        top++;
        if (tree <= forest[i][col])
        {
            break;
        }
    }
    // check right
    int right = 0;
    for (int i = col + 1; i < size; i++)
    {
        right++;
        if (tree <= forest[row][i])
        {
            break;
        }
    }
    // check left
    int left = 0;
    for (int i = col - 1; i >= 0; i--)
    {
        left++;
        if (tree <= forest[row][i])
        {
            break;
        }
    }
    return top * bottom * right * left;
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
        int highScore = -1;
        for (int tempRow = 1; tempRow < size - 1; tempRow++)
        {
            for (int tempCol = 1; tempCol < size - 1; tempCol++)
            {
                int score = scanDirections(forest, tempRow, tempCol);
                if (score > highScore)
                {
                    highScore = score;
                }
            }
        }
        cout << highScore;
    }
    file.close();
    return 0;
}
