#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <queue>

using namespace std;

using namespace std;

vector<string> split(string line)
{
    line = line.substr(1, line.size() - 2);
    vector<string> data;
    int depth = 0;
    string tmp = "";

    for (int i = 0; i < line.size(); i++)
    {
        if (line[i] == '[')
            depth++;
        else if (line[i] == ']')
            depth--;
        else if (line[i] == ',' && depth == 0)
        {
            data.push_back(tmp);
            tmp.clear();
        }

        if (line[i] != ',' || line[i] == ',' && depth != 0)
        {
            tmp.push_back(line[i]);
        }
    }
    if (tmp.size() != 0)
    {
        data.push_back(tmp);
    }
    return data;
}

int compare(vector<string> left, vector<string> right)
{
    for (int i = 0; i < min(left.size(), right.size()); i++)
    {
        if (left[i][0] != '[' && right[i][0] != '[')
        {
            if (stoi(left[i]) - stoi(right[i]) != 0)
            {
                return stoi(left[i]) - stoi(right[i]);
            }
        }
        else if (left[i][0] == '[' && right[i][0] != '[')
        {
            int x = compare(split(left[i]), split("[" + right[i] + "]"));
            if (x != 0)
            {
                return x;
            }
        }
        else if (left[i][0] != '[' && right[i][0] == '[')
        {
            int x = compare(split("[" + left[i] + "]"), split(right[i]));
            if (x != 0)
            {
                return x;
            }
        }
        else
        {
            int x = compare(split(left[i]), split(right[i]));
            if (x != 0)
            {
                return x;
            }
        }
    }
    return left.size() - right.size();
}

int main()
{
    ifstream file;
    file.open("input.txt");
    if (file.is_open())
    {
        string line1, line2;
        // add the two separators
        vector<string> vec = {"[[2]]", "[[6]]"};
        while (getline(file, line1))
        {
            getline(file, line2);
            vec.push_back(line1);
            vec.push_back(line2);
            // get blank line
            getline(file, line1);
        }
        sort(vec.begin(), vec.end(), [](string s1, string s2)
             { return compare(split(s1), split(s2)) < 0; });

        cout << (find(vec.begin(), vec.end(), "[[2]]") - vec.begin() + 1) * (find(vec.begin(), vec.end(), "[[6]]") - vec.begin() + 1);
    }
    file.close();
    return 0;
}