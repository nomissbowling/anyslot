/// bridge.cpp

#include <iomanip>
#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <map>

#include <cstring>
#include <cmath>

using namespace std;

#include <bridge.hpp>

/// universal area bridge globals
static bridge_global **bg = NULL;

/// bridge global init slots
unsigned int bridge_global_init_slots(size_t m)
{
  if(bg) return 0;
  bg = (bridge_global **)malloc((m + 1) * sizeof(bridge_global *));
  if(bg){
    bg[0] = (bridge_global *)malloc(sizeof(bridge_global));
    if(!bg[0]){ free(bg); bg = NULL; return 0; }
    bg[0]->num = m;
    return 1;
  }else{
    return 0;
  }
}

/// bridge global dispose slots
unsigned int bridge_global_dispose_slots()
{
  if(!bg) return 0;
  if(bg[0]){ free(bg[0]); bg[0] = NULL; }
  free(bg); bg = NULL;
  return 1;
}

/// private check index
static unsigned int bridge_global_check_index(size_t n)
{
  if(!bg) return 0;
  if(n >= bg[0]->num) return 0;
  return 1;
}

/// universal area bridge global setter
unsigned int bridge_global_setter(size_t n, bridge_global *p)
{
  if(!bridge_global_check_index(n)) return 0;
  bg[n + 1] = p;
  return 1;
}

/// universal area bridge global getter
bridge_global *bridge_global_getter(size_t n)
{
  if(!bridge_global_check_index(n)) return NULL;
  return bg[n + 1];
}
