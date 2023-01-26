#include <iostream>
#include <fstream>
#include <algorithm>
#include <vector>

using namespace std;

int main()
{
	vector<int> elfs;
	int totalCalories = 0;
	ifstream file;
	file.open("input.txt");
	if (file.is_open())
	{
		string line;
		while (getline(file, line))
		{
			if (line != "")
			{
				totalCalories += stoi(line);
				continue;
			}
			elfs.push_back(totalCalories);
			totalCalories = 0;
		}
	}
	file.close();
	cout << "Max calories = " << *max_element(elfs.begin(), elfs.end());
	return 0;
}
