/// This file is generated from generate.py.
/// Re-generate it if you upgrade to a new version of RocksDB.

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum DBStatisticsTickerType {
    BlockCacheMiss = 0,
    BlockCacheHit = 1,
    BlockCacheAdd = 2,
    BlockCacheAddFailures = 3,
    BlockCacheIndexMiss = 4,
    BlockCacheIndexHit = 5,
    BlockCacheIndexAdd = 6,
    BlockCacheIndexBytesInsert = 7,
    BlockCacheFilterMiss = 8,
    BlockCacheFilterHit = 9,
    BlockCacheFilterAdd = 10,
    BlockCacheFilterBytesInsert = 11,
    BlockCacheDataMiss = 12,
    BlockCacheDataHit = 13,
    BlockCacheDataAdd = 14,
    BlockCacheDataBytesInsert = 15,
    BlockCacheBytesRead = 16,
    BlockCacheBytesWrite = 17,
    BloomFilterUseful = 18,
    BloomFilterFullPositive = 19,
    BloomFilterFullTruePositive = 20,
    PersistentCacheHit = 21,
    PersistentCacheMiss = 22,
    SimBlockCacheHit = 23,
    SimBlockCacheMiss = 24,
    MemtableHit = 25,
    MemtableMiss = 26,
    GetHitL0 = 27,
    GetHitL1 = 28,
    GetHitL2AndUp = 29,
    CompactionKeyDropNewerEntry = 30,
    CompactionKeyDropObsolete = 31,
    CompactionKeyDropRangeDel = 32,
    CompactionKeyDropUser = 33,
    CompactionRangeDelDropObsolete = 34,
    CompactionOptimizedDelDropObsolete = 35,
    CompactionCancelled = 36,
    NumberKeysWritten = 37,
    NumberKeysRead = 38,
    NumberKeysUpdated = 39,
    BytesWritten = 40,
    BytesRead = 41,
    NumberDbSeek = 42,
    NumberDbNext = 43,
    NumberDbPrev = 44,
    NumberDbSeekFound = 45,
    NumberDbNextFound = 46,
    NumberDbPrevFound = 47,
    IterBytesRead = 48,
    NoFileOpens = 49,
    NoFileErrors = 50,
    StallMicros = 51,
    DbMutexWaitMicros = 52,
    NumberMultigetCalls = 53,
    NumberMultigetKeysRead = 54,
    NumberMultigetBytesRead = 55,
    NumberMergeFailures = 56,
    BloomFilterPrefixChecked = 57,
    BloomFilterPrefixUseful = 58,
    BloomFilterPrefixTruePositive = 59,
    NumberOfReseeksInIteration = 60,
    GetUpdatesSinceCalls = 61,
    WalFileSynced = 62,
    WalFileBytes = 63,
    WriteDoneBySelf = 64,
    WriteDoneByOther = 65,
    WriteWithWal = 66,
    CompactReadBytes = 67,
    CompactWriteBytes = 68,
    FlushWriteBytes = 69,
    CompactReadBytesMarked = 70,
    CompactReadBytesPeriodic = 71,
    CompactReadBytesTtl = 72,
    CompactWriteBytesMarked = 73,
    CompactWriteBytesPeriodic = 74,
    CompactWriteBytesTtl = 75,
    NumberDirectLoadTableProperties = 76,
    NumberSuperversionAcquires = 77,
    NumberSuperversionReleases = 78,
    NumberSuperversionCleanups = 79,
    NumberBlockCompressed = 80,
    NumberBlockDecompressed = 81,
    NumberBlockNotCompressed = 82,
    MergeOperationTotalTime = 83,
    FilterOperationTotalTime = 84,
    RowCacheHit = 85,
    RowCacheMiss = 86,
    ReadAmpEstimateUsefulBytes = 87,
    ReadAmpTotalReadBytes = 88,
    NumberRateLimiterDrains = 89,
    NumberIterSkip = 90,
    BlobDbNumPut = 91,
    BlobDbNumWrite = 92,
    BlobDbNumGet = 93,
    BlobDbNumMultiget = 94,
    BlobDbNumSeek = 95,
    BlobDbNumNext = 96,
    BlobDbNumPrev = 97,
    BlobDbNumKeysWritten = 98,
    BlobDbNumKeysRead = 99,
    BlobDbBytesWritten = 100,
    BlobDbBytesRead = 101,
    BlobDbWriteInlined = 102,
    BlobDbWriteInlinedTtl = 103,
    BlobDbWriteBlob = 104,
    BlobDbWriteBlobTtl = 105,
    BlobDbBlobFileBytesWritten = 106,
    BlobDbBlobFileBytesRead = 107,
    BlobDbBlobFileSynced = 108,
    BlobDbBlobIndexExpiredCount = 109,
    BlobDbBlobIndexExpiredSize = 110,
    BlobDbBlobIndexEvictedCount = 111,
    BlobDbBlobIndexEvictedSize = 112,
    BlobDbGcNumFiles = 113,
    BlobDbGcNumNewFiles = 114,
    BlobDbGcFailures = 115,
    BlobDbGcNumKeysRelocated = 116,
    BlobDbGcBytesRelocated = 117,
    BlobDbFifoNumFilesEvicted = 118,
    BlobDbFifoNumKeysEvicted = 119,
    BlobDbFifoBytesEvicted = 120,
    TxnPrepareMutexOverhead = 121,
    TxnOldCommitMapMutexOverhead = 122,
    TxnDuplicateKeyOverhead = 123,
    TxnSnapshotMutexOverhead = 124,
    TxnGetTryAgain = 125,
    NumberMultigetKeysFound = 126,
    NoIteratorCreated = 127,
    NoIteratorDeleted = 128,
    BlockCacheCompressionDictMiss = 129,
    BlockCacheCompressionDictHit = 130,
    BlockCacheCompressionDictAdd = 131,
    BlockCacheCompressionDictBytesInsert = 132,
    BlockCacheAddRedundant = 133,
    BlockCacheIndexAddRedundant = 134,
    BlockCacheFilterAddRedundant = 135,
    BlockCacheDataAddRedundant = 136,
    BlockCacheCompressionDictAddRedundant = 137,
    FilesMarkedTrash = 138,
    FilesDeletedFromTrashQueue = 139,
    FilesDeletedImmediately = 140,
    ErrorHandlerBgErrorCount = 141,
    ErrorHandlerBgErrorCountMisspelled = 142,
    ErrorHandlerBgIoErrorCount = 143,
    ErrorHandlerBgIoErrorCountMisspelled = 144,
    ErrorHandlerBgRetryableIoErrorCount = 145,
    ErrorHandlerBgRetryableIoErrorCountMisspelled = 146,
    ErrorHandlerAutoresumeCount = 147,
    ErrorHandlerAutoresumeRetryTotalCount = 148,
    ErrorHandlerAutoresumeSuccessCount = 149,
    MemtablePayloadBytesAtFlush = 150,
    MemtableGarbageBytesAtFlush = 151,
    SecondaryCacheHits = 152,
    VerifyChecksumReadBytes = 153,
    BackupReadBytes = 154,
    BackupWriteBytes = 155,
    RemoteCompactReadBytes = 156,
    RemoteCompactWriteBytes = 157,
    HotFileReadBytes = 158,
    WarmFileReadBytes = 159,
    ColdFileReadBytes = 160,
    HotFileReadCount = 161,
    WarmFileReadCount = 162,
    ColdFileReadCount = 163,
    LastLevelReadBytes = 164,
    LastLevelReadCount = 165,
    NonLastLevelReadBytes = 166,
    NonLastLevelReadCount = 167,
    LastLevelSeekFiltered = 168,
    LastLevelSeekFilterMatch = 169,
    LastLevelSeekData = 170,
    LastLevelSeekDataUsefulNoFilter = 171,
    LastLevelSeekDataUsefulFilterMatch = 172,
    NonLastLevelSeekFiltered = 173,
    NonLastLevelSeekFilterMatch = 174,
    NonLastLevelSeekData = 175,
    NonLastLevelSeekDataUsefulNoFilter = 176,
    NonLastLevelSeekDataUsefulFilterMatch = 177,
    BlockChecksumComputeCount = 178,
    BlockChecksumMismatchCount = 179,
    MultigetCoroutineCount = 180,
    BlobDbCacheMiss = 181,
    BlobDbCacheHit = 182,
    BlobDbCacheAdd = 183,
    BlobDbCacheAddFailures = 184,
    BlobDbCacheBytesRead = 185,
    BlobDbCacheBytesWrite = 186,
    ReadAsyncMicros = 187,
    AsyncReadErrorCount = 188,
    SecondaryCacheFilterHits = 189,
    SecondaryCacheIndexHits = 190,
    SecondaryCacheDataHits = 191,
    TableOpenPrefetchTailMiss = 192,
    TableOpenPrefetchTailHit = 193,
    TimestampFilterTableChecked = 194,
    TimestampFilterTableFiltered = 195,
    BytesCompressedFrom = 196,
    BytesCompressedTo = 197,
    BytesCompressionBypassed = 198,
    BytesCompressionRejected = 199,
    NumberBlockCompressionBypassed = 200,
    NumberBlockCompressionRejected = 201,
    BytesDecompressedFrom = 202,
    BytesDecompressedTo = 203,
    ReadaheadTrimmed = 204,
}
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(C)]
pub enum DBStatisticsHistogramType {
    DbGet = 0,
    DbWrite = 1,
    CompactionTime = 2,
    CompactionCpuTime = 3,
    SubcompactionSetupTime = 4,
    TableSyncMicros = 5,
    CompactionOutfileSyncMicros = 6,
    WalFileSyncMicros = 7,
    ManifestFileSyncMicros = 8,
    TableOpenIoMicros = 9,
    DbMultiget = 10,
    ReadBlockCompactionMicros = 11,
    ReadBlockGetMicros = 12,
    WriteRawBlockMicros = 13,
    NumFilesInSingleCompaction = 14,
    DbSeek = 15,
    WriteStall = 16,
    SstReadMicros = 17,
    FileReadFlushMicros = 18,
    FileReadCompactionMicros = 19,
    FileReadDbOpenMicros = 20,
    FileReadGetMicros = 21,
    FileReadMultigetMicros = 22,
    FileReadDbIteratorMicros = 23,
    FileReadVerifyDbChecksumMicros = 24,
    FileReadVerifyFileChecksumsMicros = 25,
    NumSubcompactionsScheduled = 26,
    BytesPerRead = 27,
    BytesPerWrite = 28,
    BytesPerMultiget = 29,
    BytesCompressed = 30,
    BytesDecompressed = 31,
    CompressionTimesNanos = 32,
    DecompressionTimesNanos = 33,
    ReadNumMergeOperands = 34,
    BlobDbKeySize = 35,
    BlobDbValueSize = 36,
    BlobDbWriteMicros = 37,
    BlobDbGetMicros = 38,
    BlobDbMultigetMicros = 39,
    BlobDbSeekMicros = 40,
    BlobDbNextMicros = 41,
    BlobDbPrevMicros = 42,
    BlobDbBlobFileWriteMicros = 43,
    BlobDbBlobFileReadMicros = 44,
    BlobDbBlobFileSyncMicros = 45,
    BlobDbCompressionMicros = 46,
    BlobDbDecompressionMicros = 47,
    FlushTime = 48,
    SstBatchSize = 49,
    NumIndexAndFilterBlocksReadPerLevel = 50,
    NumSstReadPerLevel = 51,
    ErrorHandlerAutoresumeRetryCount = 52,
    AsyncReadBytes = 53,
    PollWaitMicros = 54,
    PrefetchedBytesDiscarded = 55,
    MultigetIoBatchSize = 56,
    NumLevelReadPerMultiget = 57,
    AsyncPrefetchAbortMicros = 58,
    DbGetMemtable = 59,
    DbWalWriteTime = 60,
    DbWriteWaitForWal = 61,
    DbWriteWaitForWalWithMutex = 62,
    TableOpenPrefetchTailReadBytes = 63,
}
