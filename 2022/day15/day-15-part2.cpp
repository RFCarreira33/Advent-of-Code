#include <algorithm>
#include <fstream>
#include <iostream>
#include <iterator>
#include <queue>
#include <set>
#include <stdio.h>
#include <string>
#include <utility>
#include <vector>

using namespace std;

typedef pair<int, int> Coordenates;

const long Y = 4000000;

int sign(int x, int y) { return (x > y) - (x < y); }

int stepsToBeacon(int sx, int sy, int bx, int by) {
  int steps = 0;

  while (sx != bx || sy != by) {
    if (sx != bx) {
      sx += sign(bx, sx);
      steps++;
    }
    if (sy != by) {
      sy += sign(by, sy);
      steps++;
    }
  }
  return steps;
}

// Wierd indentation due to nvim auto formatting

int main() {
  ifstream file;
  file.open("input.txt");
  if (file.is_open()) {
    string line;
    vector<Coordenates> sensors;
    vector<int> stepsOfSensor;
    while (getline(file, line)) {
      // parse the input
      int sx, sy, bx, by;
      sscanf_s(line.c_str(),
               "Sensor at x=%d, y=%d: closest beacon is at x=%d, y=%d", &sx,
               &sy, &bx, &by);

      int steps = stepsToBeacon(sx, sy, bx, by);
      sensors.push_back({sx, sy});
      stepsOfSensor.push_back(steps);
    }
    int ax = -1, ay = -1;
    for (int k = 0; k <= Y && ax == -1; k++) {
      vector<Coordenates> r;
      for (int s = 0; s < sensors.size(); s++) {
        int x = sensors[s].first, y = sensors[s].second,
            steps = stepsOfSensor[s], dis = abs(y - k);
        if (dis <= steps) {
          int left = steps - dis;
          if (left >= 0) {
            r.push_back({x - left, x + left});
          }
        }
      }
      sort(r.begin(), r.end());
      // half reddit solution
      int p = 0;
      for (int i = 1; i < r.size(); i++)
        if (r[p].second >= r[i].first) {
          r[p].second = max(r[p].second, r[i].second);
        } else {
          r[++p] = r[i];
        }
      if (p > 0) {
        ax = r[0].second + 1, ay = k;
      }
    }
    // Couldnt get the correct answer because of the int limit
    // so I multiplied the below formula externally
    cout << ax << " * 4000000 + " << ay << endl;
  }
  file.close();
  return 0;
}
