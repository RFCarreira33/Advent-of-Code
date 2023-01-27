#include <iostream>
#include <fstream>
#include <algorithm>
#include <map>
#include <vector>

using namespace std;

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
            vector<int> sectors;
            int firstbegin, firstend, secondbegin, secondend;
            sscanf(line.c_str(), "%d-%d,%d-%d", &firstbegin, &firstend, &secondbegin, &secondend);
            for (size_t i = firstbegin; i <= firstend; i++)
            {
                sectors.push_back(i);
            }
            for (size_t i = secondbegin; i <= secondend; i++)
            {
                if (find(sectors.begin(), sectors.end(), i) != sectors.end())
                {
                    sum++;
                    break;
                }
            }
        }
    }
    file.close();
    cout << "The total of pairs is " << sum;
    return 0;
}