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

const int LINE = 2000000;

typedef pair<int, int> Coordenates;

int sign(int x, int y) { return (x > y) - (x < y); }

int stepsToBeacon(int sx, int sy, int bx, int by){
  int steps = 0;

  while (sx != bx || sy != by) {
    if(sx != bx){
      sx += sign(bx, sx);
      steps++;
    }
    if(sy != by){
      sy += sign(by, sy);
      steps++;
    }
  }
  return steps;
}

int main() {
  ifstream file;
  file.open("input.txt");
  if (file.is_open()) {
    string line;
    set<Coordenates> members;
    set<Coordenates> beaconsAtLine;
    while (getline(file, line)) {
      // parse the input
      int sx, sy, bx, by;
      sscanf_s(line.c_str(), "Sensor at x=%d, y=%d: closest beacon is at x=%d, y=%d", &sx, &sy, &bx, &by);
      // if the beacon is we can subtract him from final result
      if(by == LINE){
        beaconsAtLine.insert(Coordenates(bx, by));
      }
      // the area each beacon covers is a circle
      // we can see what area each radar covers with how many steps they need to reach the beacon
      int steps = stepsToBeacon(sx, sy, bx, by);

      // and then we figure out what points are covered by the radar
      steps -= abs(sy - LINE);
      if(steps >= 0){
        // duplicated values will not be added because of the set
        for (int i = steps; i >= 0; i--) {
          members.insert(Coordenates(sx - i, LINE)); 
          members.insert(Coordenates(sx + i, LINE)); 
        }
      }
    }
    // not the most optimal solution tho
    cout << members.size() - beaconsAtLine.size() << endl;
  }
  file.close();
  return 0;
}
