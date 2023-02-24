#include <algorithm>
#include <fstream>
#include <iostream>
#include <iterator>
#include <queue>
#include <set>
#include <stdio.h>
#include <string>
#include <vector>

using namespace std;

typedef pair<int, int> Coordenates;

// Indentation is looking wierd because I got my nvim working with cpp
// wish I had found this function earlier thanks stackoverflow!

vector<string> split(string s, string token = " ") {
  vector<string> sparts;
  int start, end = -1 * token.size();

  do {
    start = end + token.size();
    end = s.find(token, start);
    sparts.push_back(s.substr(start, end - start));
  } while (end != -1);

  return sparts;
}

int sign(int x, int y) { return (x > y) - (x < y); }

void fillCave(set<Coordenates> &cave, int x1, int y1, int x2, int y2) {
  if (x1 != x2) {
    while (x1 != x2) {
      cave.insert(Coordenates(x1, y1));
      x1 += sign(x2, x1);
    }
  }

  if (y1 != y2) {
    while (y1 != y2) {
      cave.insert(Coordenates(x1, y1));
      y1 += sign(y2, y1);
    }
  }
  cave.insert(Coordenates(x2, y2));
}

bool simulateSand(set<Coordenates> &cave, int maxy) {
  Coordenates source = Coordenates(500, 0);
  while (source.second <= maxy) {
    // move to the next position until the sand stops
    if (cave.find({source.first, source.second + 1}) == cave.end()) {
      source.second++;
      continue;
    }
    if (cave.find({source.first - 1, source.second + 1}) == cave.end()) {
      source.first--;
      source.second++;
      continue;
    }
    if (cave.find({source.first + 1, source.second + 1}) == cave.end()) {
      source.first++;
      source.second++;
      continue;
    }
    cave.insert(source);
    return true;
  }
  // if the y axis is > max y axis it has fallen into to the void
  return false;
}

int main() {
  ifstream file;
  file.open("input.txt");
  if (file.is_open()) {
    string line;
    set<Coordenates> cave;
    int x1, y1, x2, y2;
    vector<string> lineCoords;
    int maxy = 0;
    // parse the input
    while (getline(file, line)) {
      lineCoords = split(line, " -> ");
      for (int i = 0; i < lineCoords.size() - 1; i++) {
        sscanf_s(lineCoords[i].c_str(), "%d,%d", &x1, &y1);
        sscanf_s(lineCoords[i + 1].c_str(), "%d,%d", &x2, &y2);
        if (maxy < y1) {
          maxy = y1;
        }
        // get all the rock positions into the set
        fillCave(cave, x1, y1, x2, y2);
      }
    }
    // -1 because because I stop late
    int count = -1;
    bool notVoid = true;
    while (notVoid) {
      notVoid = simulateSand(cave, maxy);
      count++;
    }
    cout << count;
  }
  file.close();
  return 0;
}
