#include <Carbon/Carbon.h>

bool is_shift_pressed()
{
  return CGEventSourceKeyState(kCGEventSourceStateHIDSystemState, kVK_Shift) == 1;
}

bool is_command_pressed()
{
  return CGEventSourceKeyState(kCGEventSourceStateHIDSystemState, kVK_Command) == 1;
}

bool is_control_pressed()
{
  return CGEventSourceKeyState(kCGEventSourceStateHIDSystemState, kVK_Control) == 1;
}

bool is_option_pressed()
{
  return CGEventSourceKeyState(kCGEventSourceStateHIDSystemState, kVK_Option) == 1;
}