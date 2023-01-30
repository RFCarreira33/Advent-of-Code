#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

using namespace std;

const int MARKER_SIZE = 4;

bool check(string slice)
{
    vector<char> vec;
    for (int i = 0; i < MARKER_SIZE; i++)
    {
        char c = slice[i];
        if (find(vec.begin(), vec.end(), c) != vec.end())
        {
            return false;
        }
        vec.push_back(c);
    }
    return true;
}

int main()
{
    ifstream file;
    file.open("input.txt");
    if (file.is_open())
    {
        string line;
        getline(file, line);
        for (int i = 0; i < line.length(); i++)
        {
            string slice = line.substr(i, i + MARKER_SIZE);
            if (check(slice))
            {
                cout << i + MARKER_SIZE;
                return 0;
            }
        }
    }
    file.close();
    return 0;
}