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
        string line1, line2, line3;
        vector<string> groups;
        while (getline(file, line1))
        {
            getline(file, line2);
            getline(file, line3);
            map<char, bool> duplicateds;
            map<char, bool> doubleDuplicateds;
            // divides line and stores
            for (char c : line1)
            {
                duplicateds[c] = true;
            }
            for (char c : line2)
            {
                if (duplicateds[c])
                {
                    doubleDuplicateds[c] = true;
                    duplicateds.erase(c);
                }
            }
            for (char c : line3)
            {
                if (doubleDuplicateds[c])
                {
                    sum += getLetterScore(c);
                    doubleDuplicateds.erase(c);
                }
            }
        }
    }
    file.close();
    cout << "The total sum " << sum;
    return 0;
}