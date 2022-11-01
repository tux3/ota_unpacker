// Copyright 2017 The Chromium OS Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#ifndef SRC_SINK_STREAM_H_
#define SRC_SINK_STREAM_H_

#include <string>
#include <utility>

#include "puffin/common.h"
#include "puffin/stream.h"

namespace puffin {

// A very simple class for writing through a sink callback.
class SinkStream : public StreamInterface {
 public:
  ~SinkStream() override = default;
  static UniqueStreamPtr Create(const void* userarg, uint64_t(*sink)(const void* userarg, const uint8_t*, uint64_t));

  bool GetSize(uint64_t* size) const override;
  bool GetOffset(uint64_t* offset) const override;
  bool Seek(uint64_t offset) override;
  bool Read(void* buffer, size_t length) override;
  bool Write(const void* buffer, size_t length) override;
  bool Close() override;

 private:
  // Ctor.
  SinkStream(const void* userarg, uint64_t(*sink)(const void* userarg, const uint8_t*, uint64_t));

  const void* userarg_;
  uint64_t(*sink_)(const void* userarg, const uint8_t*, uint64_t);

  // The current offset.
  uint64_t offset_;

  // True if the stream is open.
  bool open_;

  DISALLOW_COPY_AND_ASSIGN(SinkStream);
};

}  // namespace puffin

#endif  // SRC_SINK_STREAM_H_
