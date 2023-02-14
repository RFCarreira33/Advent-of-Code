#include <algorithm>
#include <fstream>
#include <iostream>
#include <queue>
#include <string>
#include <vector>

using namespace std;

vector<string> split(string line) {
  line = line.substr(1, line.size() - 2);
  vector<string> data;
  int depth = 0;
  string tmp = "";

  for (int i = 0; i < line.size(); i++) {
    if (line[i] == '[')
      depth++;
    else if (line[i] == ']')
      depth--;
    else if (line[i] == ',' && depth == 0) {
      data.push_back(tmp);
      tmp.clear();
    }

    if (line[i] != ',' || line[i] == ',' && depth != 0) {
      tmp.push_back(line[i]);
    }
  }
  if (tmp.size() != 0) {
    data.push_back(tmp);
  }
  return data;
}

int compare(vector<string> left, vector<string> right) {
  for (int i = 0; i < min(left.size(), right.size()); i++) {
    if (left[i][0] != '[' && right[i][0] != '[') {
      if (stoi(left[i]) - stoi(right[i]) != 0) {
        return stoi(left[i]) - stoi(right[i]);
      }
    } else if (left[i][0] == '[' && right[i][0] != '[') {
      int x = compare(split(left[i]), split("[" + right[i] + "]"));
      if (x != 0) {
        return x;
      }
    } else if (left[i][0] != '[' && right[i][0] == '[') {
      int x = compare(split("[" + left[i] + "]"), split(right[i]));
      if (x != 0) {
        return x;
      }
    } else {
      int x = compare(split(left[i]), split(right[i]));
      if (x != 0) {
        return x;
      }
    }
  }
  return left.size() - right.size();
}

int main() {
  ifstream file;
  file.open("input.txt");
  if (file.is_open()) {
    string line1, line2;
    int sum = 0, index = 1;
    while (getline(file, line1)) {
      getline(file, line2);
      if (compare(split(line1), split(line2)) < 0) {
        sum += index;
      }
      index++;
      // get the blank line
      getline(file, line1);
    }
    cout << sum;
  }
  file.close();
  return 0;
}
