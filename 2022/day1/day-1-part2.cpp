#include <iostream>
#include <fstream>
#include <algorithm>
#include <numeric>
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
	sort(elfs.begin(), elfs.end());
	cout << "Top 3 calories total = " << accumulate(elfs.end() - 3, elfs.end(), 0);
	return 0;
}
