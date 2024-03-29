// Generated by the protocol buffer compiler.  DO NOT EDIT!
// source: puffin.proto

#ifndef GOOGLE_PROTOBUF_INCLUDED_puffin_2eproto
#define GOOGLE_PROTOBUF_INCLUDED_puffin_2eproto

#include <limits>
#include <string>

#include <google/protobuf/port_def.inc>
#if PROTOBUF_VERSION < 3021000
#error This file was generated by a newer version of protoc which is
#error incompatible with your Protocol Buffer headers. Please update
#error your headers.
#endif
#if 3021012 < PROTOBUF_MIN_PROTOC_VERSION
#error This file was generated by an older version of protoc which is
#error incompatible with your Protocol Buffer headers. Please
#error regenerate this file with a newer version of protoc.
#endif

#include <google/protobuf/port_undef.inc>
#include <google/protobuf/io/coded_stream.h>
#include <google/protobuf/arena.h>
#include <google/protobuf/arenastring.h>
#include <google/protobuf/generated_message_util.h>
#include <google/protobuf/metadata_lite.h>
#include <google/protobuf/message_lite.h>
#include <google/protobuf/repeated_field.h>  // IWYU pragma: export
#include <google/protobuf/extension_set.h>  // IWYU pragma: export
#include <google/protobuf/generated_enum_util.h>
// @@protoc_insertion_point(includes)
#include <google/protobuf/port_def.inc>
#define PROTOBUF_INTERNAL_EXPORT_puffin_2eproto
PROTOBUF_NAMESPACE_OPEN
namespace internal {
class AnyMetadata;
}  // namespace internal
PROTOBUF_NAMESPACE_CLOSE

// Internal implementation detail -- do not use these members.
struct TableStruct_puffin_2eproto {
  static const uint32_t offsets[];
};
namespace puffin {
namespace metadata {
class BitExtent;
struct BitExtentDefaultTypeInternal;
extern BitExtentDefaultTypeInternal _BitExtent_default_instance_;
class PatchHeader;
struct PatchHeaderDefaultTypeInternal;
extern PatchHeaderDefaultTypeInternal _PatchHeader_default_instance_;
class StreamInfo;
struct StreamInfoDefaultTypeInternal;
extern StreamInfoDefaultTypeInternal _StreamInfo_default_instance_;
}  // namespace metadata
}  // namespace puffin
PROTOBUF_NAMESPACE_OPEN
template<> ::puffin::metadata::BitExtent* Arena::CreateMaybeMessage<::puffin::metadata::BitExtent>(Arena*);
template<> ::puffin::metadata::PatchHeader* Arena::CreateMaybeMessage<::puffin::metadata::PatchHeader>(Arena*);
template<> ::puffin::metadata::StreamInfo* Arena::CreateMaybeMessage<::puffin::metadata::StreamInfo>(Arena*);
PROTOBUF_NAMESPACE_CLOSE
namespace puffin {
namespace metadata {

enum PatchHeader_PatchType : int {
  PatchHeader_PatchType_BSDIFF = 0,
  PatchHeader_PatchType_ZUCCHINI = 1,
  PatchHeader_PatchType_PatchHeader_PatchType_INT_MIN_SENTINEL_DO_NOT_USE_ = std::numeric_limits<int32_t>::min(),
  PatchHeader_PatchType_PatchHeader_PatchType_INT_MAX_SENTINEL_DO_NOT_USE_ = std::numeric_limits<int32_t>::max()
};
bool PatchHeader_PatchType_IsValid(int value);
constexpr PatchHeader_PatchType PatchHeader_PatchType_PatchType_MIN = PatchHeader_PatchType_BSDIFF;
constexpr PatchHeader_PatchType PatchHeader_PatchType_PatchType_MAX = PatchHeader_PatchType_ZUCCHINI;
constexpr int PatchHeader_PatchType_PatchType_ARRAYSIZE = PatchHeader_PatchType_PatchType_MAX + 1;

const std::string& PatchHeader_PatchType_Name(PatchHeader_PatchType value);
template<typename T>
inline const std::string& PatchHeader_PatchType_Name(T enum_t_value) {
  static_assert(::std::is_same<T, PatchHeader_PatchType>::value ||
    ::std::is_integral<T>::value,
    "Incorrect type passed to function PatchHeader_PatchType_Name.");
  return PatchHeader_PatchType_Name(static_cast<PatchHeader_PatchType>(enum_t_value));
}
bool PatchHeader_PatchType_Parse(
    ::PROTOBUF_NAMESPACE_ID::ConstStringParam name, PatchHeader_PatchType* value);
// ===================================================================

class BitExtent final :
    public ::PROTOBUF_NAMESPACE_ID::MessageLite /* @@protoc_insertion_point(class_definition:puffin.metadata.BitExtent) */ {
 public:
  inline BitExtent() : BitExtent(nullptr) {}
  ~BitExtent() override;
  explicit PROTOBUF_CONSTEXPR BitExtent(::PROTOBUF_NAMESPACE_ID::internal::ConstantInitialized);

  BitExtent(const BitExtent& from);
  BitExtent(BitExtent&& from) noexcept
    : BitExtent() {
    *this = ::std::move(from);
  }

  inline BitExtent& operator=(const BitExtent& from) {
    CopyFrom(from);
    return *this;
  }
  inline BitExtent& operator=(BitExtent&& from) noexcept {
    if (this == &from) return *this;
    if (GetOwningArena() == from.GetOwningArena()
  #ifdef PROTOBUF_FORCE_COPY_IN_MOVE
        && GetOwningArena() != nullptr
  #endif  // !PROTOBUF_FORCE_COPY_IN_MOVE
    ) {
      InternalSwap(&from);
    } else {
      CopyFrom(from);
    }
    return *this;
  }

  static const BitExtent& default_instance() {
    return *internal_default_instance();
  }
  static inline const BitExtent* internal_default_instance() {
    return reinterpret_cast<const BitExtent*>(
               &_BitExtent_default_instance_);
  }
  static constexpr int kIndexInFileMessages =
    0;

  friend void swap(BitExtent& a, BitExtent& b) {
    a.Swap(&b);
  }
  inline void Swap(BitExtent* other) {
    if (other == this) return;
  #ifdef PROTOBUF_FORCE_COPY_IN_SWAP
    if (GetOwningArena() != nullptr &&
        GetOwningArena() == other->GetOwningArena()) {
   #else  // PROTOBUF_FORCE_COPY_IN_SWAP
    if (GetOwningArena() == other->GetOwningArena()) {
  #endif  // !PROTOBUF_FORCE_COPY_IN_SWAP
      InternalSwap(other);
    } else {
      ::PROTOBUF_NAMESPACE_ID::internal::GenericSwap(this, other);
    }
  }
  void UnsafeArenaSwap(BitExtent* other) {
    if (other == this) return;
    GOOGLE_DCHECK(GetOwningArena() == other->GetOwningArena());
    InternalSwap(other);
  }

  // implements Message ----------------------------------------------

  BitExtent* New(::PROTOBUF_NAMESPACE_ID::Arena* arena = nullptr) const final {
    return CreateMaybeMessage<BitExtent>(arena);
  }
  void CheckTypeAndMergeFrom(const ::PROTOBUF_NAMESPACE_ID::MessageLite& from)  final;
  void CopyFrom(const BitExtent& from);
  void MergeFrom(const BitExtent& from);
  PROTOBUF_ATTRIBUTE_REINITIALIZES void Clear() final;
  bool IsInitialized() const final;

  size_t ByteSizeLong() const final;
  const char* _InternalParse(const char* ptr, ::PROTOBUF_NAMESPACE_ID::internal::ParseContext* ctx) final;
  uint8_t* _InternalSerialize(
      uint8_t* target, ::PROTOBUF_NAMESPACE_ID::io::EpsCopyOutputStream* stream) const final;
  int GetCachedSize() const final { return _impl_._cached_size_.Get(); }

  private:
  void SharedCtor(::PROTOBUF_NAMESPACE_ID::Arena* arena, bool is_message_owned);
  void SharedDtor();
  void SetCachedSize(int size) const;
  void InternalSwap(BitExtent* other);

  private:
  friend class ::PROTOBUF_NAMESPACE_ID::internal::AnyMetadata;
  static ::PROTOBUF_NAMESPACE_ID::StringPiece FullMessageName() {
    return "puffin.metadata.BitExtent";
  }
  protected:
  explicit BitExtent(::PROTOBUF_NAMESPACE_ID::Arena* arena,
                       bool is_message_owned = false);
  public:

  std::string GetTypeName() const final;

  // nested types ----------------------------------------------------

  // accessors -------------------------------------------------------

  enum : int {
    kOffsetFieldNumber = 1,
    kLengthFieldNumber = 2,
  };
  // uint64 offset = 1;
  void clear_offset();
  uint64_t offset() const;
  void set_offset(uint64_t value);
  private:
  uint64_t _internal_offset() const;
  void _internal_set_offset(uint64_t value);
  public:

  // uint64 length = 2;
  void clear_length();
  uint64_t length() const;
  void set_length(uint64_t value);
  private:
  uint64_t _internal_length() const;
  void _internal_set_length(uint64_t value);
  public:

  // @@protoc_insertion_point(class_scope:puffin.metadata.BitExtent)
 private:
  class _Internal;

  template <typename T> friend class ::PROTOBUF_NAMESPACE_ID::Arena::InternalHelper;
  typedef void InternalArenaConstructable_;
  typedef void DestructorSkippable_;
  struct Impl_ {
    uint64_t offset_;
    uint64_t length_;
    mutable ::PROTOBUF_NAMESPACE_ID::internal::CachedSize _cached_size_;
  };
  union { Impl_ _impl_; };
  friend struct ::TableStruct_puffin_2eproto;
};
// -------------------------------------------------------------------

class StreamInfo final :
    public ::PROTOBUF_NAMESPACE_ID::MessageLite /* @@protoc_insertion_point(class_definition:puffin.metadata.StreamInfo) */ {
 public:
  inline StreamInfo() : StreamInfo(nullptr) {}
  ~StreamInfo() override;
  explicit PROTOBUF_CONSTEXPR StreamInfo(::PROTOBUF_NAMESPACE_ID::internal::ConstantInitialized);

  StreamInfo(const StreamInfo& from);
  StreamInfo(StreamInfo&& from) noexcept
    : StreamInfo() {
    *this = ::std::move(from);
  }

  inline StreamInfo& operator=(const StreamInfo& from) {
    CopyFrom(from);
    return *this;
  }
  inline StreamInfo& operator=(StreamInfo&& from) noexcept {
    if (this == &from) return *this;
    if (GetOwningArena() == from.GetOwningArena()
  #ifdef PROTOBUF_FORCE_COPY_IN_MOVE
        && GetOwningArena() != nullptr
  #endif  // !PROTOBUF_FORCE_COPY_IN_MOVE
    ) {
      InternalSwap(&from);
    } else {
      CopyFrom(from);
    }
    return *this;
  }

  static const StreamInfo& default_instance() {
    return *internal_default_instance();
  }
  static inline const StreamInfo* internal_default_instance() {
    return reinterpret_cast<const StreamInfo*>(
               &_StreamInfo_default_instance_);
  }
  static constexpr int kIndexInFileMessages =
    1;

  friend void swap(StreamInfo& a, StreamInfo& b) {
    a.Swap(&b);
  }
  inline void Swap(StreamInfo* other) {
    if (other == this) return;
  #ifdef PROTOBUF_FORCE_COPY_IN_SWAP
    if (GetOwningArena() != nullptr &&
        GetOwningArena() == other->GetOwningArena()) {
   #else  // PROTOBUF_FORCE_COPY_IN_SWAP
    if (GetOwningArena() == other->GetOwningArena()) {
  #endif  // !PROTOBUF_FORCE_COPY_IN_SWAP
      InternalSwap(other);
    } else {
      ::PROTOBUF_NAMESPACE_ID::internal::GenericSwap(this, other);
    }
  }
  void UnsafeArenaSwap(StreamInfo* other) {
    if (other == this) return;
    GOOGLE_DCHECK(GetOwningArena() == other->GetOwningArena());
    InternalSwap(other);
  }

  // implements Message ----------------------------------------------

  StreamInfo* New(::PROTOBUF_NAMESPACE_ID::Arena* arena = nullptr) const final {
    return CreateMaybeMessage<StreamInfo>(arena);
  }
  void CheckTypeAndMergeFrom(const ::PROTOBUF_NAMESPACE_ID::MessageLite& from)  final;
  void CopyFrom(const StreamInfo& from);
  void MergeFrom(const StreamInfo& from);
  PROTOBUF_ATTRIBUTE_REINITIALIZES void Clear() final;
  bool IsInitialized() const final;

  size_t ByteSizeLong() const final;
  const char* _InternalParse(const char* ptr, ::PROTOBUF_NAMESPACE_ID::internal::ParseContext* ctx) final;
  uint8_t* _InternalSerialize(
      uint8_t* target, ::PROTOBUF_NAMESPACE_ID::io::EpsCopyOutputStream* stream) const final;
  int GetCachedSize() const final { return _impl_._cached_size_.Get(); }

  private:
  void SharedCtor(::PROTOBUF_NAMESPACE_ID::Arena* arena, bool is_message_owned);
  void SharedDtor();
  void SetCachedSize(int size) const;
  void InternalSwap(StreamInfo* other);

  private:
  friend class ::PROTOBUF_NAMESPACE_ID::internal::AnyMetadata;
  static ::PROTOBUF_NAMESPACE_ID::StringPiece FullMessageName() {
    return "puffin.metadata.StreamInfo";
  }
  protected:
  explicit StreamInfo(::PROTOBUF_NAMESPACE_ID::Arena* arena,
                       bool is_message_owned = false);
  public:

  std::string GetTypeName() const final;

  // nested types ----------------------------------------------------

  // accessors -------------------------------------------------------

  enum : int {
    kDeflatesFieldNumber = 1,
    kPuffsFieldNumber = 2,
    kPuffLengthFieldNumber = 3,
  };
  // repeated .puffin.metadata.BitExtent deflates = 1;
  int deflates_size() const;
  private:
  int _internal_deflates_size() const;
  public:
  void clear_deflates();
  ::puffin::metadata::BitExtent* mutable_deflates(int index);
  ::PROTOBUF_NAMESPACE_ID::RepeatedPtrField< ::puffin::metadata::BitExtent >*
      mutable_deflates();
  private:
  const ::puffin::metadata::BitExtent& _internal_deflates(int index) const;
  ::puffin::metadata::BitExtent* _internal_add_deflates();
  public:
  const ::puffin::metadata::BitExtent& deflates(int index) const;
  ::puffin::metadata::BitExtent* add_deflates();
  const ::PROTOBUF_NAMESPACE_ID::RepeatedPtrField< ::puffin::metadata::BitExtent >&
      deflates() const;

  // repeated .puffin.metadata.BitExtent puffs = 2;
  int puffs_size() const;
  private:
  int _internal_puffs_size() const;
  public:
  void clear_puffs();
  ::puffin::metadata::BitExtent* mutable_puffs(int index);
  ::PROTOBUF_NAMESPACE_ID::RepeatedPtrField< ::puffin::metadata::BitExtent >*
      mutable_puffs();
  private:
  const ::puffin::metadata::BitExtent& _internal_puffs(int index) const;
  ::puffin::metadata::BitExtent* _internal_add_puffs();
  public:
  const ::puffin::metadata::BitExtent& puffs(int index) const;
  ::puffin::metadata::BitExtent* add_puffs();
  const ::PROTOBUF_NAMESPACE_ID::RepeatedPtrField< ::puffin::metadata::BitExtent >&
      puffs() const;

  // uint64 puff_length = 3;
  void clear_puff_length();
  uint64_t puff_length() const;
  void set_puff_length(uint64_t value);
  private:
  uint64_t _internal_puff_length() const;
  void _internal_set_puff_length(uint64_t value);
  public:

  // @@protoc_insertion_point(class_scope:puffin.metadata.StreamInfo)
 private:
  class _Internal;

  template <typename T> friend class ::PROTOBUF_NAMESPACE_ID::Arena::InternalHelper;
  typedef void InternalArenaConstructable_;
  typedef void DestructorSkippable_;
  struct Impl_ {
    ::PROTOBUF_NAMESPACE_ID::RepeatedPtrField< ::puffin::metadata::BitExtent > deflates_;
    ::PROTOBUF_NAMESPACE_ID::RepeatedPtrField< ::puffin::metadata::BitExtent > puffs_;
    uint64_t puff_length_;
    mutable ::PROTOBUF_NAMESPACE_ID::internal::CachedSize _cached_size_;
  };
  union { Impl_ _impl_; };
  friend struct ::TableStruct_puffin_2eproto;
};
// -------------------------------------------------------------------

class PatchHeader final :
    public ::PROTOBUF_NAMESPACE_ID::MessageLite /* @@protoc_insertion_point(class_definition:puffin.metadata.PatchHeader) */ {
 public:
  inline PatchHeader() : PatchHeader(nullptr) {}
  ~PatchHeader() override;
  explicit PROTOBUF_CONSTEXPR PatchHeader(::PROTOBUF_NAMESPACE_ID::internal::ConstantInitialized);

  PatchHeader(const PatchHeader& from);
  PatchHeader(PatchHeader&& from) noexcept
    : PatchHeader() {
    *this = ::std::move(from);
  }

  inline PatchHeader& operator=(const PatchHeader& from) {
    CopyFrom(from);
    return *this;
  }
  inline PatchHeader& operator=(PatchHeader&& from) noexcept {
    if (this == &from) return *this;
    if (GetOwningArena() == from.GetOwningArena()
  #ifdef PROTOBUF_FORCE_COPY_IN_MOVE
        && GetOwningArena() != nullptr
  #endif  // !PROTOBUF_FORCE_COPY_IN_MOVE
    ) {
      InternalSwap(&from);
    } else {
      CopyFrom(from);
    }
    return *this;
  }

  static const PatchHeader& default_instance() {
    return *internal_default_instance();
  }
  static inline const PatchHeader* internal_default_instance() {
    return reinterpret_cast<const PatchHeader*>(
               &_PatchHeader_default_instance_);
  }
  static constexpr int kIndexInFileMessages =
    2;

  friend void swap(PatchHeader& a, PatchHeader& b) {
    a.Swap(&b);
  }
  inline void Swap(PatchHeader* other) {
    if (other == this) return;
  #ifdef PROTOBUF_FORCE_COPY_IN_SWAP
    if (GetOwningArena() != nullptr &&
        GetOwningArena() == other->GetOwningArena()) {
   #else  // PROTOBUF_FORCE_COPY_IN_SWAP
    if (GetOwningArena() == other->GetOwningArena()) {
  #endif  // !PROTOBUF_FORCE_COPY_IN_SWAP
      InternalSwap(other);
    } else {
      ::PROTOBUF_NAMESPACE_ID::internal::GenericSwap(this, other);
    }
  }
  void UnsafeArenaSwap(PatchHeader* other) {
    if (other == this) return;
    GOOGLE_DCHECK(GetOwningArena() == other->GetOwningArena());
    InternalSwap(other);
  }

  // implements Message ----------------------------------------------

  PatchHeader* New(::PROTOBUF_NAMESPACE_ID::Arena* arena = nullptr) const final {
    return CreateMaybeMessage<PatchHeader>(arena);
  }
  void CheckTypeAndMergeFrom(const ::PROTOBUF_NAMESPACE_ID::MessageLite& from)  final;
  void CopyFrom(const PatchHeader& from);
  void MergeFrom(const PatchHeader& from);
  PROTOBUF_ATTRIBUTE_REINITIALIZES void Clear() final;
  bool IsInitialized() const final;

  size_t ByteSizeLong() const final;
  const char* _InternalParse(const char* ptr, ::PROTOBUF_NAMESPACE_ID::internal::ParseContext* ctx) final;
  uint8_t* _InternalSerialize(
      uint8_t* target, ::PROTOBUF_NAMESPACE_ID::io::EpsCopyOutputStream* stream) const final;
  int GetCachedSize() const final { return _impl_._cached_size_.Get(); }

  private:
  void SharedCtor(::PROTOBUF_NAMESPACE_ID::Arena* arena, bool is_message_owned);
  void SharedDtor();
  void SetCachedSize(int size) const;
  void InternalSwap(PatchHeader* other);

  private:
  friend class ::PROTOBUF_NAMESPACE_ID::internal::AnyMetadata;
  static ::PROTOBUF_NAMESPACE_ID::StringPiece FullMessageName() {
    return "puffin.metadata.PatchHeader";
  }
  protected:
  explicit PatchHeader(::PROTOBUF_NAMESPACE_ID::Arena* arena,
                       bool is_message_owned = false);
  public:

  std::string GetTypeName() const final;

  // nested types ----------------------------------------------------

  typedef PatchHeader_PatchType PatchType;
  static constexpr PatchType BSDIFF =
    PatchHeader_PatchType_BSDIFF;
  static constexpr PatchType ZUCCHINI =
    PatchHeader_PatchType_ZUCCHINI;
  static inline bool PatchType_IsValid(int value) {
    return PatchHeader_PatchType_IsValid(value);
  }
  static constexpr PatchType PatchType_MIN =
    PatchHeader_PatchType_PatchType_MIN;
  static constexpr PatchType PatchType_MAX =
    PatchHeader_PatchType_PatchType_MAX;
  static constexpr int PatchType_ARRAYSIZE =
    PatchHeader_PatchType_PatchType_ARRAYSIZE;
  template<typename T>
  static inline const std::string& PatchType_Name(T enum_t_value) {
    static_assert(::std::is_same<T, PatchType>::value ||
      ::std::is_integral<T>::value,
      "Incorrect type passed to function PatchType_Name.");
    return PatchHeader_PatchType_Name(enum_t_value);
  }
  static inline bool PatchType_Parse(::PROTOBUF_NAMESPACE_ID::ConstStringParam name,
      PatchType* value) {
    return PatchHeader_PatchType_Parse(name, value);
  }

  // accessors -------------------------------------------------------

  enum : int {
    kSrcFieldNumber = 2,
    kDstFieldNumber = 3,
    kVersionFieldNumber = 1,
    kTypeFieldNumber = 4,
  };
  // .puffin.metadata.StreamInfo src = 2;
  bool has_src() const;
  private:
  bool _internal_has_src() const;
  public:
  void clear_src();
  const ::puffin::metadata::StreamInfo& src() const;
  PROTOBUF_NODISCARD ::puffin::metadata::StreamInfo* release_src();
  ::puffin::metadata::StreamInfo* mutable_src();
  void set_allocated_src(::puffin::metadata::StreamInfo* src);
  private:
  const ::puffin::metadata::StreamInfo& _internal_src() const;
  ::puffin::metadata::StreamInfo* _internal_mutable_src();
  public:
  void unsafe_arena_set_allocated_src(
      ::puffin::metadata::StreamInfo* src);
  ::puffin::metadata::StreamInfo* unsafe_arena_release_src();

  // .puffin.metadata.StreamInfo dst = 3;
  bool has_dst() const;
  private:
  bool _internal_has_dst() const;
  public:
  void clear_dst();
  const ::puffin::metadata::StreamInfo& dst() const;
  PROTOBUF_NODISCARD ::puffin::metadata::StreamInfo* release_dst();
  ::puffin::metadata::StreamInfo* mutable_dst();
  void set_allocated_dst(::puffin::metadata::StreamInfo* dst);
  private:
  const ::puffin::metadata::StreamInfo& _internal_dst() const;
  ::puffin::metadata::StreamInfo* _internal_mutable_dst();
  public:
  void unsafe_arena_set_allocated_dst(
      ::puffin::metadata::StreamInfo* dst);
  ::puffin::metadata::StreamInfo* unsafe_arena_release_dst();

  // int32 version = 1;
  void clear_version();
  int32_t version() const;
  void set_version(int32_t value);
  private:
  int32_t _internal_version() const;
  void _internal_set_version(int32_t value);
  public:

  // .puffin.metadata.PatchHeader.PatchType type = 4;
  void clear_type();
  ::puffin::metadata::PatchHeader_PatchType type() const;
  void set_type(::puffin::metadata::PatchHeader_PatchType value);
  private:
  ::puffin::metadata::PatchHeader_PatchType _internal_type() const;
  void _internal_set_type(::puffin::metadata::PatchHeader_PatchType value);
  public:

  // @@protoc_insertion_point(class_scope:puffin.metadata.PatchHeader)
 private:
  class _Internal;

  template <typename T> friend class ::PROTOBUF_NAMESPACE_ID::Arena::InternalHelper;
  typedef void InternalArenaConstructable_;
  typedef void DestructorSkippable_;
  struct Impl_ {
    ::puffin::metadata::StreamInfo* src_;
    ::puffin::metadata::StreamInfo* dst_;
    int32_t version_;
    int type_;
    mutable ::PROTOBUF_NAMESPACE_ID::internal::CachedSize _cached_size_;
  };
  union { Impl_ _impl_; };
  friend struct ::TableStruct_puffin_2eproto;
};
// ===================================================================


// ===================================================================

#ifdef __GNUC__
  #pragma GCC diagnostic push
  #pragma GCC diagnostic ignored "-Wstrict-aliasing"
#endif  // __GNUC__
// BitExtent

// uint64 offset = 1;
inline void BitExtent::clear_offset() {
  _impl_.offset_ = uint64_t{0u};
}
inline uint64_t BitExtent::_internal_offset() const {
  return _impl_.offset_;
}
inline uint64_t BitExtent::offset() const {
  // @@protoc_insertion_point(field_get:puffin.metadata.BitExtent.offset)
  return _internal_offset();
}
inline void BitExtent::_internal_set_offset(uint64_t value) {
  
  _impl_.offset_ = value;
}
inline void BitExtent::set_offset(uint64_t value) {
  _internal_set_offset(value);
  // @@protoc_insertion_point(field_set:puffin.metadata.BitExtent.offset)
}

// uint64 length = 2;
inline void BitExtent::clear_length() {
  _impl_.length_ = uint64_t{0u};
}
inline uint64_t BitExtent::_internal_length() const {
  return _impl_.length_;
}
inline uint64_t BitExtent::length() const {
  // @@protoc_insertion_point(field_get:puffin.metadata.BitExtent.length)
  return _internal_length();
}
inline void BitExtent::_internal_set_length(uint64_t value) {
  
  _impl_.length_ = value;
}
inline void BitExtent::set_length(uint64_t value) {
  _internal_set_length(value);
  // @@protoc_insertion_point(field_set:puffin.metadata.BitExtent.length)
}

// -------------------------------------------------------------------

// StreamInfo

// repeated .puffin.metadata.BitExtent deflates = 1;
inline int StreamInfo::_internal_deflates_size() const {
  return _impl_.deflates_.size();
}
inline int StreamInfo::deflates_size() const {
  return _internal_deflates_size();
}
inline void StreamInfo::clear_deflates() {
  _impl_.deflates_.Clear();
}
inline ::puffin::metadata::BitExtent* StreamInfo::mutable_deflates(int index) {
  // @@protoc_insertion_point(field_mutable:puffin.metadata.StreamInfo.deflates)
  return _impl_.deflates_.Mutable(index);
}
inline ::PROTOBUF_NAMESPACE_ID::RepeatedPtrField< ::puffin::metadata::BitExtent >*
StreamInfo::mutable_deflates() {
  // @@protoc_insertion_point(field_mutable_list:puffin.metadata.StreamInfo.deflates)
  return &_impl_.deflates_;
}
inline const ::puffin::metadata::BitExtent& StreamInfo::_internal_deflates(int index) const {
  return _impl_.deflates_.Get(index);
}
inline const ::puffin::metadata::BitExtent& StreamInfo::deflates(int index) const {
  // @@protoc_insertion_point(field_get:puffin.metadata.StreamInfo.deflates)
  return _internal_deflates(index);
}
inline ::puffin::metadata::BitExtent* StreamInfo::_internal_add_deflates() {
  return _impl_.deflates_.Add();
}
inline ::puffin::metadata::BitExtent* StreamInfo::add_deflates() {
  ::puffin::metadata::BitExtent* _add = _internal_add_deflates();
  // @@protoc_insertion_point(field_add:puffin.metadata.StreamInfo.deflates)
  return _add;
}
inline const ::PROTOBUF_NAMESPACE_ID::RepeatedPtrField< ::puffin::metadata::BitExtent >&
StreamInfo::deflates() const {
  // @@protoc_insertion_point(field_list:puffin.metadata.StreamInfo.deflates)
  return _impl_.deflates_;
}

// repeated .puffin.metadata.BitExtent puffs = 2;
inline int StreamInfo::_internal_puffs_size() const {
  return _impl_.puffs_.size();
}
inline int StreamInfo::puffs_size() const {
  return _internal_puffs_size();
}
inline void StreamInfo::clear_puffs() {
  _impl_.puffs_.Clear();
}
inline ::puffin::metadata::BitExtent* StreamInfo::mutable_puffs(int index) {
  // @@protoc_insertion_point(field_mutable:puffin.metadata.StreamInfo.puffs)
  return _impl_.puffs_.Mutable(index);
}
inline ::PROTOBUF_NAMESPACE_ID::RepeatedPtrField< ::puffin::metadata::BitExtent >*
StreamInfo::mutable_puffs() {
  // @@protoc_insertion_point(field_mutable_list:puffin.metadata.StreamInfo.puffs)
  return &_impl_.puffs_;
}
inline const ::puffin::metadata::BitExtent& StreamInfo::_internal_puffs(int index) const {
  return _impl_.puffs_.Get(index);
}
inline const ::puffin::metadata::BitExtent& StreamInfo::puffs(int index) const {
  // @@protoc_insertion_point(field_get:puffin.metadata.StreamInfo.puffs)
  return _internal_puffs(index);
}
inline ::puffin::metadata::BitExtent* StreamInfo::_internal_add_puffs() {
  return _impl_.puffs_.Add();
}
inline ::puffin::metadata::BitExtent* StreamInfo::add_puffs() {
  ::puffin::metadata::BitExtent* _add = _internal_add_puffs();
  // @@protoc_insertion_point(field_add:puffin.metadata.StreamInfo.puffs)
  return _add;
}
inline const ::PROTOBUF_NAMESPACE_ID::RepeatedPtrField< ::puffin::metadata::BitExtent >&
StreamInfo::puffs() const {
  // @@protoc_insertion_point(field_list:puffin.metadata.StreamInfo.puffs)
  return _impl_.puffs_;
}

// uint64 puff_length = 3;
inline void StreamInfo::clear_puff_length() {
  _impl_.puff_length_ = uint64_t{0u};
}
inline uint64_t StreamInfo::_internal_puff_length() const {
  return _impl_.puff_length_;
}
inline uint64_t StreamInfo::puff_length() const {
  // @@protoc_insertion_point(field_get:puffin.metadata.StreamInfo.puff_length)
  return _internal_puff_length();
}
inline void StreamInfo::_internal_set_puff_length(uint64_t value) {
  
  _impl_.puff_length_ = value;
}
inline void StreamInfo::set_puff_length(uint64_t value) {
  _internal_set_puff_length(value);
  // @@protoc_insertion_point(field_set:puffin.metadata.StreamInfo.puff_length)
}

// -------------------------------------------------------------------

// PatchHeader

// int32 version = 1;
inline void PatchHeader::clear_version() {
  _impl_.version_ = 0;
}
inline int32_t PatchHeader::_internal_version() const {
  return _impl_.version_;
}
inline int32_t PatchHeader::version() const {
  // @@protoc_insertion_point(field_get:puffin.metadata.PatchHeader.version)
  return _internal_version();
}
inline void PatchHeader::_internal_set_version(int32_t value) {
  
  _impl_.version_ = value;
}
inline void PatchHeader::set_version(int32_t value) {
  _internal_set_version(value);
  // @@protoc_insertion_point(field_set:puffin.metadata.PatchHeader.version)
}

// .puffin.metadata.StreamInfo src = 2;
inline bool PatchHeader::_internal_has_src() const {
  return this != internal_default_instance() && _impl_.src_ != nullptr;
}
inline bool PatchHeader::has_src() const {
  return _internal_has_src();
}
inline void PatchHeader::clear_src() {
  if (GetArenaForAllocation() == nullptr && _impl_.src_ != nullptr) {
    delete _impl_.src_;
  }
  _impl_.src_ = nullptr;
}
inline const ::puffin::metadata::StreamInfo& PatchHeader::_internal_src() const {
  const ::puffin::metadata::StreamInfo* p = _impl_.src_;
  return p != nullptr ? *p : reinterpret_cast<const ::puffin::metadata::StreamInfo&>(
      ::puffin::metadata::_StreamInfo_default_instance_);
}
inline const ::puffin::metadata::StreamInfo& PatchHeader::src() const {
  // @@protoc_insertion_point(field_get:puffin.metadata.PatchHeader.src)
  return _internal_src();
}
inline void PatchHeader::unsafe_arena_set_allocated_src(
    ::puffin::metadata::StreamInfo* src) {
  if (GetArenaForAllocation() == nullptr) {
    delete reinterpret_cast<::PROTOBUF_NAMESPACE_ID::MessageLite*>(_impl_.src_);
  }
  _impl_.src_ = src;
  if (src) {
    
  } else {
    
  }
  // @@protoc_insertion_point(field_unsafe_arena_set_allocated:puffin.metadata.PatchHeader.src)
}
inline ::puffin::metadata::StreamInfo* PatchHeader::release_src() {
  
  ::puffin::metadata::StreamInfo* temp = _impl_.src_;
  _impl_.src_ = nullptr;
#ifdef PROTOBUF_FORCE_COPY_IN_RELEASE
  auto* old =  reinterpret_cast<::PROTOBUF_NAMESPACE_ID::MessageLite*>(temp);
  temp = ::PROTOBUF_NAMESPACE_ID::internal::DuplicateIfNonNull(temp);
  if (GetArenaForAllocation() == nullptr) { delete old; }
#else  // PROTOBUF_FORCE_COPY_IN_RELEASE
  if (GetArenaForAllocation() != nullptr) {
    temp = ::PROTOBUF_NAMESPACE_ID::internal::DuplicateIfNonNull(temp);
  }
#endif  // !PROTOBUF_FORCE_COPY_IN_RELEASE
  return temp;
}
inline ::puffin::metadata::StreamInfo* PatchHeader::unsafe_arena_release_src() {
  // @@protoc_insertion_point(field_release:puffin.metadata.PatchHeader.src)
  
  ::puffin::metadata::StreamInfo* temp = _impl_.src_;
  _impl_.src_ = nullptr;
  return temp;
}
inline ::puffin::metadata::StreamInfo* PatchHeader::_internal_mutable_src() {
  
  if (_impl_.src_ == nullptr) {
    auto* p = CreateMaybeMessage<::puffin::metadata::StreamInfo>(GetArenaForAllocation());
    _impl_.src_ = p;
  }
  return _impl_.src_;
}
inline ::puffin::metadata::StreamInfo* PatchHeader::mutable_src() {
  ::puffin::metadata::StreamInfo* _msg = _internal_mutable_src();
  // @@protoc_insertion_point(field_mutable:puffin.metadata.PatchHeader.src)
  return _msg;
}
inline void PatchHeader::set_allocated_src(::puffin::metadata::StreamInfo* src) {
  ::PROTOBUF_NAMESPACE_ID::Arena* message_arena = GetArenaForAllocation();
  if (message_arena == nullptr) {
    delete _impl_.src_;
  }
  if (src) {
    ::PROTOBUF_NAMESPACE_ID::Arena* submessage_arena =
        ::PROTOBUF_NAMESPACE_ID::Arena::InternalGetOwningArena(src);
    if (message_arena != submessage_arena) {
      src = ::PROTOBUF_NAMESPACE_ID::internal::GetOwnedMessage(
          message_arena, src, submessage_arena);
    }
    
  } else {
    
  }
  _impl_.src_ = src;
  // @@protoc_insertion_point(field_set_allocated:puffin.metadata.PatchHeader.src)
}

// .puffin.metadata.StreamInfo dst = 3;
inline bool PatchHeader::_internal_has_dst() const {
  return this != internal_default_instance() && _impl_.dst_ != nullptr;
}
inline bool PatchHeader::has_dst() const {
  return _internal_has_dst();
}
inline void PatchHeader::clear_dst() {
  if (GetArenaForAllocation() == nullptr && _impl_.dst_ != nullptr) {
    delete _impl_.dst_;
  }
  _impl_.dst_ = nullptr;
}
inline const ::puffin::metadata::StreamInfo& PatchHeader::_internal_dst() const {
  const ::puffin::metadata::StreamInfo* p = _impl_.dst_;
  return p != nullptr ? *p : reinterpret_cast<const ::puffin::metadata::StreamInfo&>(
      ::puffin::metadata::_StreamInfo_default_instance_);
}
inline const ::puffin::metadata::StreamInfo& PatchHeader::dst() const {
  // @@protoc_insertion_point(field_get:puffin.metadata.PatchHeader.dst)
  return _internal_dst();
}
inline void PatchHeader::unsafe_arena_set_allocated_dst(
    ::puffin::metadata::StreamInfo* dst) {
  if (GetArenaForAllocation() == nullptr) {
    delete reinterpret_cast<::PROTOBUF_NAMESPACE_ID::MessageLite*>(_impl_.dst_);
  }
  _impl_.dst_ = dst;
  if (dst) {
    
  } else {
    
  }
  // @@protoc_insertion_point(field_unsafe_arena_set_allocated:puffin.metadata.PatchHeader.dst)
}
inline ::puffin::metadata::StreamInfo* PatchHeader::release_dst() {
  
  ::puffin::metadata::StreamInfo* temp = _impl_.dst_;
  _impl_.dst_ = nullptr;
#ifdef PROTOBUF_FORCE_COPY_IN_RELEASE
  auto* old =  reinterpret_cast<::PROTOBUF_NAMESPACE_ID::MessageLite*>(temp);
  temp = ::PROTOBUF_NAMESPACE_ID::internal::DuplicateIfNonNull(temp);
  if (GetArenaForAllocation() == nullptr) { delete old; }
#else  // PROTOBUF_FORCE_COPY_IN_RELEASE
  if (GetArenaForAllocation() != nullptr) {
    temp = ::PROTOBUF_NAMESPACE_ID::internal::DuplicateIfNonNull(temp);
  }
#endif  // !PROTOBUF_FORCE_COPY_IN_RELEASE
  return temp;
}
inline ::puffin::metadata::StreamInfo* PatchHeader::unsafe_arena_release_dst() {
  // @@protoc_insertion_point(field_release:puffin.metadata.PatchHeader.dst)
  
  ::puffin::metadata::StreamInfo* temp = _impl_.dst_;
  _impl_.dst_ = nullptr;
  return temp;
}
inline ::puffin::metadata::StreamInfo* PatchHeader::_internal_mutable_dst() {
  
  if (_impl_.dst_ == nullptr) {
    auto* p = CreateMaybeMessage<::puffin::metadata::StreamInfo>(GetArenaForAllocation());
    _impl_.dst_ = p;
  }
  return _impl_.dst_;
}
inline ::puffin::metadata::StreamInfo* PatchHeader::mutable_dst() {
  ::puffin::metadata::StreamInfo* _msg = _internal_mutable_dst();
  // @@protoc_insertion_point(field_mutable:puffin.metadata.PatchHeader.dst)
  return _msg;
}
inline void PatchHeader::set_allocated_dst(::puffin::metadata::StreamInfo* dst) {
  ::PROTOBUF_NAMESPACE_ID::Arena* message_arena = GetArenaForAllocation();
  if (message_arena == nullptr) {
    delete _impl_.dst_;
  }
  if (dst) {
    ::PROTOBUF_NAMESPACE_ID::Arena* submessage_arena =
        ::PROTOBUF_NAMESPACE_ID::Arena::InternalGetOwningArena(dst);
    if (message_arena != submessage_arena) {
      dst = ::PROTOBUF_NAMESPACE_ID::internal::GetOwnedMessage(
          message_arena, dst, submessage_arena);
    }
    
  } else {
    
  }
  _impl_.dst_ = dst;
  // @@protoc_insertion_point(field_set_allocated:puffin.metadata.PatchHeader.dst)
}

// .puffin.metadata.PatchHeader.PatchType type = 4;
inline void PatchHeader::clear_type() {
  _impl_.type_ = 0;
}
inline ::puffin::metadata::PatchHeader_PatchType PatchHeader::_internal_type() const {
  return static_cast< ::puffin::metadata::PatchHeader_PatchType >(_impl_.type_);
}
inline ::puffin::metadata::PatchHeader_PatchType PatchHeader::type() const {
  // @@protoc_insertion_point(field_get:puffin.metadata.PatchHeader.type)
  return _internal_type();
}
inline void PatchHeader::_internal_set_type(::puffin::metadata::PatchHeader_PatchType value) {
  
  _impl_.type_ = value;
}
inline void PatchHeader::set_type(::puffin::metadata::PatchHeader_PatchType value) {
  _internal_set_type(value);
  // @@protoc_insertion_point(field_set:puffin.metadata.PatchHeader.type)
}

#ifdef __GNUC__
  #pragma GCC diagnostic pop
#endif  // __GNUC__
// -------------------------------------------------------------------

// -------------------------------------------------------------------


// @@protoc_insertion_point(namespace_scope)

}  // namespace metadata
}  // namespace puffin

PROTOBUF_NAMESPACE_OPEN

template <> struct is_proto_enum< ::puffin::metadata::PatchHeader_PatchType> : ::std::true_type {};

PROTOBUF_NAMESPACE_CLOSE

// @@protoc_insertion_point(global_scope)

#include <google/protobuf/port_undef.inc>
#endif  // GOOGLE_PROTOBUF_INCLUDED_GOOGLE_PROTOBUF_INCLUDED_puffin_2eproto
