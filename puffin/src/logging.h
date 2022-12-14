// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_LOGGING_H_
#define SRC_LOGGING_H_

#include <string.h>

#include <iostream>
#include <sstream>

// Simple error logging macro to avoid dependencies in other base libraries.
#define LOG(severity) LogMessage(__FILE__, __LINE__, #severity).stream()

// A variant of LOG that also logs the current errno value.
#define PLOG(severity) LogMessage(__FILE__, __LINE__, #severity, errno).stream()

// A temporarily scoped object used by LOG & PLOG.
class LogMessage {
 public:
  LogMessage(const char* file, unsigned int line, const char* severity) {};

  LogMessage(const char* file,
             unsigned int line,
             const char* severity,
             int error) {};

  ~LogMessage() {};

  // Returns the stream associated with the message, the LogMessage performs
  // output when it goes out of scope.
  std::ostream& stream() { return stream_; }

 private:
  std::ostringstream stream_;
  int error_;  // The saved errno value.
};

#define TEST_AND_RETURN_FALSE(_x)   \
  do {                              \
    if (!(_x)) {                    \
      LOG(ERROR) << #_x " failed."; \
      return false;                 \
    }                               \
  } while (0)

#define TEST_AND_RETURN_VALUE(_x, _v) \
  do {                                \
    if (!(_x)) {                      \
      LOG(ERROR) << #_x " failed.";   \
      return (_v);                    \
    }                                 \
  } while (0)

#endif  // SRC_LOGGING_H_
