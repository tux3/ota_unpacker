// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#include "puffin/sink_stream.h"

#include <fcntl.h>
#include <unistd.h>

#include <algorithm>
#include <utility>
#include <cstring>

#include "puffin/src/include/puffin/common.h"
#include "puffin/src/logging.h"

namespace puffin {

UniqueStreamPtr SinkStream::Create(const void* userarg, uint64_t(*sink)(const void* userarg, const uint8_t*, uint64_t)) {
  return UniqueStreamPtr(new SinkStream(userarg, sink));
}

SinkStream::SinkStream(const void* userarg, uint64_t(*sink)(const void* userarg, const uint8_t*, uint64_t))
    : userarg_(userarg),
      sink_(sink),
      offset_(0),
      expected_write_offset_(0),
      open_(true) {}

bool SinkStream::GetSize(uint64_t* size) const {
  *size = offset_;
  return true;
}

bool SinkStream::GetOffset(uint64_t* offset) const {
  *offset = offset_;
  return true;
}

bool SinkStream::Seek(uint64_t offset) {
  TEST_AND_RETURN_FALSE(open_);

//  if (offset != offset_)
//    return false;
  offset_ = offset;
  return true;
//  uint64_t size;
//  GetSize(&size);
//  TEST_AND_RETURN_FALSE(offset <= size);
//  offset_ = offset;
//  return true;
}

bool SinkStream::Read(void* buffer, size_t length) {
  return false;
}

bool SinkStream::Write(const void* buffer, size_t length) {
  TEST_AND_RETURN_FALSE(open_);
  if (expected_write_offset_ != offset_)
    abort();
  sink_(userarg_, (const uint8_t*)buffer, length);
  offset_ += length;
  expected_write_offset_ = offset_;
  return true;
}

bool SinkStream::Close() {
  open_ = false;
  return true;
}

}  // namespace puffin
