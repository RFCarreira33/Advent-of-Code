#include <iostream>
#include <fstream>
#include <vector>
#include <map>

using namespace std;

int getLetterScore(char c)
{
    return c >= 'a' ? c - 'a' + 1 : c - 'A' + 27;
}

int main()
{
    ifstream file;
    file.open("input.txt");
    int sum = 0;
    if (file.is_open())
    {
        string line;
        while (getline(file, line))
        {
            map<char, bool> duplicateds;
            // divides line and stores
            string first = line.substr(0, line.length() / 2);
            string second = line.substr(line.length() / 2);
            for (char c : first)
            {
                duplicateds[c] = true;
            }
            for (char c : second)
            {
                if (duplicateds[c])
                {
                    sum += getLetterScore(c);
                    duplicateds.erase(c);
                }
            }
        }
    }
    file.close();
    cout << "The sum of the priorities is " << sum;
    return 0;
}