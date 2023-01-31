#include <algorithm>
#include <fstream>
#include <iostream>
#include <string>
#include <vector>
#include <map>

using namespace std;

const int MAX_SIZE = 100000;

struct Directory
{
    string name;
    struct Directory *parent;
    map<string, Directory> children;
    int size;
};

struct Directory *commandHandler(struct Directory *currentDir, string lineCommand)
{
    if (lineCommand == "ls")
    {
        return currentDir;
    }
    string newDir = lineCommand.substr(lineCommand.find(' ') + 1);
    if (newDir == "..")
    {
        currentDir->parent;
    }
    return &currentDir->children[newDir];
}

void lsHandler(struct Directory *currentDir, string flag, string name)
{
    // if flag is dir add this dir to the children of the current Dir
    if (flag == "dir")
    {
        struct Directory child = {name, currentDir, {}, 0};
        currentDir->children.insert({name, child});
    }
    // else it will be a file
    else
    {
        int size = stoi(flag);
        currentDir->size += size;
        // update parents until reaching the head aka root
        struct Directory *parent = currentDir->parent;
        while (parent != nullptr)
        {
            parent->size += size;
            parent = parent->parent;
        }
    }
}

int sumAllDirectories(struct Directory dir)
{
    int sum = 0;
    for (auto child : dir.children)
    {
        if (child.second.size <= MAX_SIZE)
        {
            sum += child.second.size;
        }
        sum += sumAllDirectories(child.second);
    }
    return sum;
}

int main()
{
    ifstream file;
    file.open("input.txt");

    // Head
    struct Directory root =
        {
            "/",
            nullptr,
            {},
            0,
        };

    // represents current dir
    struct Directory *directory = &root;

    if (file.is_open())
    {
        string line;
        // skip first line
        getline(file, line);
        while (getline(file, line))
        {
            string flag = line.substr(0, line.find(' '));
            // if flag isnt $ means that ls was ran in the line before
            if (flag == "$")
            {
                // send everything but the '$'
                string command = line.substr(line.find(' ') + 1);
                directory = commandHandler(directory, command);
                continue;
            }
            // get file or dir name
            string name = line.substr(line.find(' ') + 1);
            lsHandler(directory, flag, name);
        }
        cout << "Sum is " << sumAllDirectories(root);
    }
    file.close();
    return 0;
}