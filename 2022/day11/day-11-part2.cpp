#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <map>
#include <set>
#include <math.h>

using namespace std;

const int NUMBER_OF_ROUNDS = 10000;
const int NUMBER_OF_MONKEYS = 8;
// after losing some brain cells saw that modulos is needed
const int MODULOS = 9699690;

struct Monkey
{
    int monekyNumber;
    vector<size_t> items;
    pair<char, int> operation;
    int divisible;
    pair<int, int> MTrueFalse;
    int inspected;
};

Monkey parseMonkey(ifstream &file)
{
    string line;
    // parse the monkey number
    int monkeyNumber;
    getline(file, line);
    if (line == "")
    {
        getline(file, line);
    }

    sscanf(line.c_str(), "Monkey %d:", &monkeyNumber);
    // parse the items
    getline(file, line);
    vector<size_t> items;
    line = line.substr(line.find(": ") + 2);
    do
    {
        items.push_back(stoi(line.substr(0, line.find(", "))));
        line = line.substr(line.find(", ") + 2);
    } while (line.find(", ") != line.npos);
    if (line.length() == 2)
    {
        items.push_back(stoi(line));
    }

    // parse the operation
    // indentation is key
    getline(file, line);
    int opNum = 0;
    char op = line.substr(line.find("old ") + 4, 1)[0];
    string num = line.substr(line.find(op) + 1);
    try
    {
        opNum = stoi(num);
    }
    catch (const std::exception &e)
    {
    }
    pair<char, int> operation = {op, opNum};
    // parse divider
    getline(file, line);
    int diviser;
    sscanf(line.c_str(), "  Test: divisible by %d", &diviser);
    // parse monkey true
    int MTrue;
    getline(file, line);
    sscanf(line.c_str(), "      If true: throw to monkey %d", &MTrue);
    // parse monkey false
    int MFalse;
    getline(file, line);
    sscanf(line.c_str(), "      If false: throw to monkey %d", &MFalse);
    pair<int, int> TFmonkeys = {MTrue, MFalse};
    // create monkey
    Monkey m =
        {
            monkeyNumber,
            items,
            operation,
            diviser,
            TFmonkeys,
            0,
        };

    return m;
}

void runRound(map<int, Monkey> &Monkeys)
{
    for (auto &m : Monkeys)
    {
        // stop looping a monkey with no items
        if (m.second.items.size() == 0)
        {
            continue;
        }

        // inspects the item
        for (size_t &item : m.second.items)
        {
            m.second.inspected++;
            // if the mult is 0 means we need to operate number is the item
            int mult = m.second.operation.second;
            switch (m.second.operation.first)
            {
            case '+':
                if (mult == 0)
                {
                    item *= item;
                    break;
                }
                item += mult;
                break;

            case '*':
                if (mult == 0)
                {
                    item *= item;
                    break;
                }
                item *= mult;
                break;
            }
            item %= MODULOS;
            if (item % m.second.divisible == 0)
            {
                Monkeys.at(m.second.MTrueFalse.first).items.push_back(item);
                continue;
            }
            Monkeys.at(m.second.MTrueFalse.second).items.push_back(item);
        }
        m.second.items.clear();
    }
}

int main()
{
    ifstream file;
    map<int, Monkey> Monkeys;
    file.open("input.txt");
    if (file.is_open())
    {
        for (int i = 0; i < NUMBER_OF_MONKEYS; i++)
        {
            Monkeys.insert({i, parseMonkey(file)});
        }
        for (int i = 0; i < NUMBER_OF_ROUNDS; i++)
        {
            runRound(Monkeys);
        }
        size_t first = 0;
        size_t second = 0;
        for (auto m : Monkeys)
        {
            if (m.second.inspected > first)
            {
                second = first;
                first = m.second.inspected;
            }
            else if (m.second.inspected > second)
            {
                second = m.second.inspected;
            }
        }
        cout << "Monkey Business = " << first * second;
    }
    file.close();
    return 0;
}