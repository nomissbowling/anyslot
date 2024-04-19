/// bridge.hpp

#ifndef __BRIDGE_H__
#define __BRIDGE_H__

/// universal area bridge global
typedef struct bridge_global {
  /// num
  size_t num;
  /// may be changed by alloc
  unsigned char buf[8];
} bridge_global;

extern "C" {
/// bridge global init slots
unsigned int bridge_global_init_slots(size_t m);
/// bridge global dispose slots
unsigned int bridge_global_dispose_slots();
/// bridge global setter
unsigned int bridge_global_setter(size_t n, bridge_global *p);
/// bridge global getter
bridge_global *bridge_global_getter(size_t n);
}

#endif // __BRIDGE_H__
