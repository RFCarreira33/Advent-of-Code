#include <iostream>
#include <fstream>
#include <algorithm>

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
            int firstbegin, firstend, secondbegin, secondend;
            sscanf(line.c_str(), "%d-%d,%d-%d", &firstbegin, &firstend, &secondbegin, &secondend);
            if (firstbegin <= secondbegin && firstend >= secondend || secondbegin <= firstbegin && secondend >= firstend)
            {
                sum++;
            }
        }
    }
    file.close();
    cout << "The total of pairs is " << sum;
    return 0;
}