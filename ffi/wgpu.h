/**
 * Copyright 2021 The gfx-rs developers
 * 
 * SPDX-License-Identifier: Apache-2.0 OR MIT
 */

#ifndef WGPU_H_
#define WGPU_H_

#if defined(WGPU_SHARED_LIBRARY)
#    if defined(_WIN32)
#        if defined(WGPU_IMPLEMENTATION)
#            define WGPU_EXPORT __declspec(dllexport)
#        else
#            define WGPU_EXPORT __declspec(dllimport)
#        endif
#    else  // defined(_WIN32)
#        if defined(WGPU_IMPLEMENTATION)
#            define WGPU_EXPORT __attribute__((visibility("default")))
#        else
#            define WGPU_EXPORT
#        endif
#    endif  // defined(_WIN32)
#else       // defined(WGPU_SHARED_LIBRARY)
#    define WGPU_EXPORT
#endif  // defined(WGPU_SHARED_LIBRARY)

#if !defined(WGPU_OBJECT_ATTRIBUTE)
#define WGPU_OBJECT_ATTRIBUTE
#endif
#if !defined(WGPU_ENUM_ATTRIBUTE)
#define WGPU_ENUM_ATTRIBUTE
#endif
#if !defined(WGPU_STRUCTURE_ATTRIBUTE)
#define WGPU_STRUCTURE_ATTRIBUTE
#endif
#if !defined(WGPU_FUNCTION_ATTRIBUTE)
#define WGPU_FUNCTION_ATTRIBUTE
#endif
#if !defined(WGPU_NULLABLE)
#define WGPU_NULLABLE
#endif

#if !defined(__WGPU_EXTEND_ENUM)
#ifdef __cplusplus
#define __WGPU_EXTEND_ENUM(E, N, V) static const E N = E(V)
#else
#define __WGPU_EXTEND_ENUM(E, N, V) static const E N = (E)(V)
#endif
#endif // !defined(__WGPU_EXTEND_ENUM)

#include "webgpu.h"

typedef uint64_t WGPUSubmissionIndexWGPU;

typedef struct WGPUComputePassEncoderWGPUImpl* WGPUComputePassEncoderWGPU WGPU_OBJECT_ATTRIBUTE;
typedef struct WGPUDeviceWGPUImpl* WGPUDeviceWGPU WGPU_OBJECT_ATTRIBUTE;
typedef struct WGPUInstanceWGPUImpl* WGPUInstanceWGPU WGPU_OBJECT_ATTRIBUTE;
typedef struct WGPUQueueWGPUImpl* WGPUQueueWGPU WGPU_OBJECT_ATTRIBUTE;
typedef struct WGPURenderPassEncoderWGPUImpl* WGPURenderPassEncoderWGPU WGPU_OBJECT_ATTRIBUTE;

// Structure forward declarations
struct WGPUBindGroupEntryExtrasWGPU;
struct WGPUBindGroupLayoutEntryExtrasWGPU;
struct WGPUDeviceExtrasWGPU;
struct WGPUInstanceEnumerateAdaptersResultWGPU;
struct WGPUInstanceExtrasWGPU;
struct WGPULimitExtrasWGPU;
struct WGPUPushConstantRangeWGPU;
struct WGPUQuerySetDescriptorExtrasWGPU;
struct WGPURegistryReportWGPU;
struct WGPUShaderDefineWGPU;
struct WGPUSurfaceConfigurationExtrasWGPU;
struct WGPUWrappedSubmissionIndexWGPU;
struct WGPUHubReportWGPU;
struct WGPURequiredLimitsExtrasWGPU;
struct WGPUShaderModuleGLSLDescriptorWGPU;
struct WGPUSupportedLimitsExtrasWGPU;
struct WGPUGlobalReportWGPU;

__WGPU_EXTEND_ENUM(WGPUFeatureName, WGPUFeatureName_PushConstants_WGPU, 0x00030000);
__WGPU_EXTEND_ENUM(WGPUFeatureName, WGPUFeatureName_TextureAdapterSpecificFormatFeatures_WGPU, 0x00030001);
__WGPU_EXTEND_ENUM(WGPUFeatureName, WGPUFeatureName_MultiDrawIndirect_WGPU, 0x00030002);
__WGPU_EXTEND_ENUM(WGPUFeatureName, WGPUFeatureName_MultiDrawIndirectCount_WGPU, 0x00030003);
__WGPU_EXTEND_ENUM(WGPUFeatureName, WGPUFeatureName_VertexWritableStorage_WGPU, 0x00030004);
__WGPU_EXTEND_ENUM(WGPUFeatureName, WGPUFeatureName_TextureBindingArray_WGPU, 0x00030005);
__WGPU_EXTEND_ENUM(WGPUFeatureName, WGPUFeatureName_SampledTextureAndStorageBufferArrayNonUniformIndexing_WGPU, 0x00030006);
__WGPU_EXTEND_ENUM(WGPUFeatureName, WGPUFeatureName_PipelineStatisticsQuery_WGPU, 0x00030007);

__WGPU_EXTEND_ENUM(WGPUQueryType, WGPUQueryType_PipelineStatistics_WGPU, 0x00030000);

__WGPU_EXTEND_ENUM(WGPUSType, WGPUSType_DeviceExtras_WGPU, 0x00030000);
__WGPU_EXTEND_ENUM(WGPUSType, WGPUSType_RequiredLimitsExtras_WGPU, 0x00030001);
__WGPU_EXTEND_ENUM(WGPUSType, WGPUSType_PipelineLayoutExtras_WGPU, 0x00030002);
__WGPU_EXTEND_ENUM(WGPUSType, WGPUSType_ShaderModuleGLSLDescriptor_WGPU, 0x00030003);
__WGPU_EXTEND_ENUM(WGPUSType, WGPUSType_SupportedLimitsExtras_WGPU, 0x00030004);
__WGPU_EXTEND_ENUM(WGPUSType, WGPUSType_InstanceExtras_WGPU, 0x00030005);
__WGPU_EXTEND_ENUM(WGPUSType, WGPUSType_BindGroupEntryExtras_WGPU, 0x00030006);
__WGPU_EXTEND_ENUM(WGPUSType, WGPUSType_BindGroupLayoutEntryExtras_WGPU, 0x00030007);
__WGPU_EXTEND_ENUM(WGPUSType, WGPUSType_QuerySetDescriptorExtras_WGPU, 0x00030008);
__WGPU_EXTEND_ENUM(WGPUSType, WGPUSType_SurfaceConfigurationExtras_WGPU, 0x00030009);

typedef enum WGPUDx12CompilerWGPU {
    WGPUDx12Compiler_Undefined_WGPU = 0x00030000,
    WGPUDx12Compiler_Fxc_WGPU = 0x00030001,
    WGPUDx12Compiler_Dxc_WGPU = 0x00030002,
    WGPUDx12Compiler_Force32_WGPU = 0x7FFFFFFF
} WGPUDx12CompilerWGPU WGPU_ENUM_ATTRIBUTE;

typedef enum WGPUGles3MinorVersionWGPU {
    WGPUGles3MinorVersion_Automatic_WGPU = 0x00030000,
    WGPUGles3MinorVersion_Version0_WGPU = 0x00030001,
    WGPUGles3MinorVersion_Version1_WGPU = 0x00030002,
    WGPUGles3MinorVersion_Version2_WGPU = 0x00030003,
    WGPUGles3MinorVersion_Force32_WGPU = 0x7FFFFFFF
} WGPUGles3MinorVersionWGPU WGPU_ENUM_ATTRIBUTE;

typedef enum WGPULogLevelWGPU {
    WGPULogLevel_Off_WGPU = 0x00030000,
    WGPULogLevel_Error_WGPU = 0x00030001,
    WGPULogLevel_Warn_WGPU = 0x00030002,
    WGPULogLevel_Info_WGPU = 0x00030003,
    WGPULogLevel_Debug_WGPU = 0x00030004,
    WGPULogLevel_Trace_WGPU = 0x00030005,
    WGPULogLevel_Force32_WGPU = 0x7FFFFFFF
} WGPULogLevelWGPU WGPU_ENUM_ATTRIBUTE;

typedef enum WGPUPipelineStatisticsNameWGPU {
    WGPUPipelineStatisticsName_VertexShaderInvocations_WGPU = 0x00030000,
    WGPUPipelineStatisticsName_ClipperInvocations_WGPU = 0x00030001,
    WGPUPipelineStatisticsName_ClipperPrimitivesOut_WGPU = 0x00030002,
    WGPUPipelineStatisticsName_FragmentShaderInvocations_WGPU = 0x00030003,
    WGPUPipelineStatisticsName_ComputeShaderInvocations_WGPU = 0x00030004,
    WGPUPipelineStatisticsName_Force32_WGPU = 0x7FFFFFFF
} WGPUPipelineStatisticsNameWGPU WGPU_ENUM_ATTRIBUTE;

typedef enum WGPUBackendsWGPU {
    WGPUBackends_Vulkan_WGPU = 0x00000000,
    WGPUBackends_GL_WGPU = 0x00000001,
    WGPUBackends_Metal_WGPU = 0x00000002,
    WGPUBackends_DX12_WGPU = 0x00000004,
    WGPUBackends_BrowserWebGPU_WGPU = 0x00000008,
    WGPUBackends_Primary_WGPU = WGPUBackends_Vulkan_WGPU | WGPUBackends_Metal_WGPU | WGPUBackends_DX12_WGPU | WGPUBackends_BrowserWebGPU_WGPU,
    WGPUBackends_Secondary_WGPU = WGPUBackends_GL_WGPU,
    WGPUBackends_Force32_WGPU = 0x7FFFFFFF
} WGPUBackendsWGPU WGPU_ENUM_ATTRIBUTE;
typedef WGPUFlags WGPUBackendsFlagsWGPU WGPU_ENUM_ATTRIBUTE;

typedef enum WGPUInstanceFlagsWGPU {
    WGPUInstanceFlags_Debug_WGPU = 0x00000000,
    WGPUInstanceFlags_Validation_WGPU = 0x00000001,
    WGPUInstanceFlags_DiscardHalLabels_WGPU = 0x00000002,
    WGPUInstanceFlags_AllowUnderlyingNoncompliantAdapter_WGPU = 0x00000004,
    WGPUInstanceFlags_Force32_WGPU = 0x7FFFFFFF
} WGPUInstanceFlagsWGPU WGPU_ENUM_ATTRIBUTE;
typedef WGPUFlags WGPUInstanceFlagsFlagsWGPU WGPU_ENUM_ATTRIBUTE;

typedef void (*WGPULogCallbackWGPU)(WGPULogLevelWGPU level, char const * message) WGPU_FUNCTION_ATTRIBUTE;


typedef struct WGPUBindGroupEntryExtrasWGPU {
    WGPUChainedStruct chain;
    size_t bufferCount;
    WGPUBuffer buffers;
    size_t samplerCount;
    WGPUSampler samplers;
    size_t textureViewCount;
    WGPUTextureView textureViews;
} WGPUBindGroupEntryExtrasWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUBindGroupLayoutEntryExtrasWGPU {
    WGPUChainedStruct chain;
    uint32_t count;
} WGPUBindGroupLayoutEntryExtrasWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUDeviceExtrasWGPU {
    WGPUChainedStruct chain;
    char const * tracePath;
} WGPUDeviceExtrasWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUInstanceEnumerateAdaptersResultWGPU {
    size_t adapterCount;
    WGPUAdapter adapters;
} WGPUInstanceEnumerateAdaptersResultWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUInstanceExtrasWGPU {
    WGPUChainedStruct chain;
    WGPUBackendsFlagsWGPU backends;
    WGPUInstanceFlagsFlagsWGPU flags;
    WGPUDx12CompilerWGPU dx12ShaderCompiler;
    WGPUGles3MinorVersionWGPU gles3MinorVersion;
    char const * dxilPath;
    char const * dxcPath;
} WGPUInstanceExtrasWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPULimitExtrasWGPU {
    uint32_t maxPushConstantSize;
    uint32_t maxNonSamplerBindings;
} WGPULimitExtrasWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUPushConstantRangeWGPU {
    WGPUShaderStageFlags stages;
    uint32_t start;
    uint32_t end;
} WGPUPushConstantRangeWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUQuerySetDescriptorExtrasWGPU {
    WGPUChainedStruct chain;
    size_t pipelineStatisticCount;
    WGPUPipelineStatisticsNameWGPU pipelineStatistics;
} WGPUQuerySetDescriptorExtrasWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPURegistryReportWGPU {
    size_t numAllocated;
    size_t numKeptFromUser;
    size_t numReleasedFromUser;
    size_t numError;
    size_t elementSize;
} WGPURegistryReportWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUShaderDefineWGPU {
    char const * name;
    char const * value;
} WGPUShaderDefineWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUSurfaceConfigurationExtrasWGPU {
    WGPUChainedStruct chain;
    WGPUBool desiredMaximumFrameLatency;
} WGPUSurfaceConfigurationExtrasWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUWrappedSubmissionIndexWGPU {
    WGPUQueue queue;
    WGPUSubmissionIndexWGPU submissionIndex;
} WGPUWrappedSubmissionIndexWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUHubReportWGPU {
    WGPURegistryReportWGPU adapters;
    WGPURegistryReportWGPU devices;
    WGPURegistryReportWGPU queues;
    WGPURegistryReportWGPU pipelineLayouts;
    WGPURegistryReportWGPU shaderModules;
    WGPURegistryReportWGPU bindGroupLayouts;
    WGPURegistryReportWGPU bindGroups;
    WGPURegistryReportWGPU commandBuffers;
    WGPURegistryReportWGPU renderBundles;
    WGPURegistryReportWGPU renderPipelines;
    WGPURegistryReportWGPU computePipelines;
    WGPURegistryReportWGPU querySets;
    WGPURegistryReportWGPU buffers;
    WGPURegistryReportWGPU textures;
    WGPURegistryReportWGPU textureViews;
    WGPURegistryReportWGPU samplers;
} WGPUHubReportWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPURequiredLimitsExtrasWGPU {
    WGPUChainedStruct chain;
    WGPULimitExtrasWGPU limits;
} WGPURequiredLimitsExtrasWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUShaderModuleGLSLDescriptorWGPU {
    WGPUChainedStruct chain;
    WGPUShaderStageFlags stage;
    char const * code;
    size_t defineCount;
    WGPUShaderDefineWGPU defines;
} WGPUShaderModuleGLSLDescriptorWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUSupportedLimitsExtrasWGPU {
    WGPUChainedStruct chain;
    WGPULimitExtrasWGPU limits;
} WGPUSupportedLimitsExtrasWGPU WGPU_STRUCTURE_ATTRIBUTE;

typedef struct WGPUGlobalReportWGPU {
    WGPURegistryReportWGPU surfaces;
    WGPUBackendType backendType;
    WGPUHubReportWGPU vulkan;
    WGPUHubReportWGPU metal;
    WGPUHubReportWGPU dx12;
    WGPUHubReportWGPU gl;
} WGPUGlobalReportWGPU WGPU_STRUCTURE_ATTRIBUTE;

#ifdef __cplusplus
extern "C" {
#endif

#if !defined(WGPU_SKIP_PROCS)

typedef void (*WGPUProcSetLogCallbackWGPU)(WGPULogCallbackWGPU callback, void * userdata) WGPU_FUNCTION_ATTRIBUTE;
typedef void (*WGPUProcSetLogLevelWGPU)(WGPULogLevelWGPU level) WGPU_FUNCTION_ATTRIBUTE;
typedef uint32_t (*WGPUProcGetVersionWGPU)() WGPU_FUNCTION_ATTRIBUTE;
// Procs of ComputePassEncoder
typedef void (*WGPUProcComputePassEncoderBeginPipelineStatisticsQueryWGPU)(WGPUComputePassEncoderWGPU computePassEncoder, WGPUQuerySet querySet, uint32_t queryIndex) WGPU_FUNCTION_ATTRIBUTE;
typedef void (*WGPUProcComputePassEncoderEndPipelineStatisticsQueryWGPU)(WGPUComputePassEncoderWGPU computePassEncoder) WGPU_FUNCTION_ATTRIBUTE;

// Procs of Device
typedef WGPUBool (*WGPUProcDevicePollWGPU)(WGPUDeviceWGPU device, WGPUBool wait, WGPUWrappedSubmissionIndexWGPU const * wrappedSubmissionIndex) WGPU_FUNCTION_ATTRIBUTE;

// Procs of Instance
typedef void (*WGPUProcInstanceEnumerateAdaptersWGPU)(WGPUInstanceWGPU instance, WGPUBackendsFlagsWGPU backends, WGPUInstanceEnumerateAdaptersResultWGPU result) WGPU_FUNCTION_ATTRIBUTE;
typedef void (*WGPUProcInstanceGenerateReportWGPU)(WGPUInstanceWGPU instance, WGPUGlobalReportWGPU * report) WGPU_FUNCTION_ATTRIBUTE;

// Procs of InstanceEnumerateAdaptersResult
typedef void (*WGPUProcInstanceEnumerateAdaptersResultFreeMembersWGPU)(WGPUInstanceEnumerateAdaptersResultWGPU instanceEnumerateAdaptersResult) WGPU_FUNCTION_ATTRIBUTE;

// Procs of Queue
typedef uint64_t (*WGPUProcQueueSubmitForIndexWGPU)(WGPUQueueWGPU queue, size_t commandCount, WGPUCommandBuffer commands) WGPU_FUNCTION_ATTRIBUTE;

// Procs of RenderPassEncoder
typedef void (*WGPUProcRenderPassEncoderBeginPipelineStatisticsQueryWGPU)(WGPURenderPassEncoderWGPU renderPassEncoder, WGPUQuerySet querySet, uint32_t queryIndex) WGPU_FUNCTION_ATTRIBUTE;
typedef void (*WGPUProcRenderPassEncoderEndPipelineStatisticsQueryWGPU)(WGPURenderPassEncoderWGPU renderPassEncoder) WGPU_FUNCTION_ATTRIBUTE;
typedef void (*WGPUProcRenderPassEncoderMultiDrawIndexedIndirectWGPU)(WGPURenderPassEncoderWGPU renderPassEncoder, WGPUBuffer indirectBuffer, uint32_t indirectOffset, size_t count) WGPU_FUNCTION_ATTRIBUTE;
typedef void (*WGPUProcRenderPassEncoderMultiDrawIndexedIndirectCountWGPU)(WGPURenderPassEncoderWGPU renderPassEncoder, WGPUBuffer indirectBuffer, uint64_t indirectOffset, WGPUBuffer countBuffer, uint64_t countOffset, uint32_t maxCount) WGPU_FUNCTION_ATTRIBUTE;
typedef void (*WGPUProcRenderPassEncoderMultiDrawIndirectWGPU)(WGPURenderPassEncoderWGPU renderPassEncoder, WGPUBuffer indirectBuffer, uint32_t indirectOffset, size_t count) WGPU_FUNCTION_ATTRIBUTE;
typedef void (*WGPUProcRenderPassEncoderMultiDrawIndirectCountWGPU)(WGPURenderPassEncoderWGPU renderPassEncoder, WGPUBuffer indirectBuffer, uint64_t indirectOffset, WGPUBuffer countBuffer, uint64_t countOffset, uint32_t maxCount) WGPU_FUNCTION_ATTRIBUTE;
typedef void (*WGPUProcRenderPassEncoderSetPushConstantsWGPU)(WGPURenderPassEncoderWGPU renderPassEncoder, WGPUShaderStageFlags stages, uint32_t offset, size_t size, void const * data) WGPU_FUNCTION_ATTRIBUTE;

#endif  // !defined(WGPU_SKIP_PROCS)

#if !defined(WGPU_SKIP_DECLARATIONS)

WGPU_EXPORT void wgpuSetLogCallbackWGPU(WGPULogCallbackWGPU callback, void * userdata) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuSetLogLevelWGPU(WGPULogLevelWGPU level) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT uint32_t wgpuGetVersionWGPU() WGPU_FUNCTION_ATTRIBUTE;
// Methods of ComputePassEncoder
WGPU_EXPORT void wgpuComputePassEncoderBeginPipelineStatisticsQueryWGPU(WGPUComputePassEncoderWGPU computePassEncoder, WGPUQuerySet querySet, uint32_t queryIndex) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuComputePassEncoderEndPipelineStatisticsQueryWGPU(WGPUComputePassEncoderWGPU computePassEncoder) WGPU_FUNCTION_ATTRIBUTE;

// Methods of Device
WGPU_EXPORT WGPUBool wgpuDevicePollWGPU(WGPUDeviceWGPU device, WGPUBool wait, WGPUWrappedSubmissionIndexWGPU const * wrappedSubmissionIndex) WGPU_FUNCTION_ATTRIBUTE;

// Methods of Instance
WGPU_EXPORT void wgpuInstanceEnumerateAdaptersWGPU(WGPUInstanceWGPU instance, WGPUBackendsFlagsWGPU backends, WGPUInstanceEnumerateAdaptersResultWGPU result) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuInstanceGenerateReportWGPU(WGPUInstanceWGPU instance, WGPUGlobalReportWGPU * report) WGPU_FUNCTION_ATTRIBUTE;

// Methods of InstanceEnumerateAdaptersResult
WGPU_EXPORT void wgpuInstanceEnumerateAdaptersResultFreeMembersWGPU(WGPUInstanceEnumerateAdaptersResultWGPU instanceEnumerateAdaptersResult) WGPU_FUNCTION_ATTRIBUTE;

// Methods of Queue
WGPU_EXPORT uint64_t wgpuQueueSubmitForIndexWGPU(WGPUQueueWGPU queue, size_t commandCount, WGPUCommandBuffer commands) WGPU_FUNCTION_ATTRIBUTE;

// Methods of RenderPassEncoder
WGPU_EXPORT void wgpuRenderPassEncoderBeginPipelineStatisticsQueryWGPU(WGPURenderPassEncoderWGPU renderPassEncoder, WGPUQuerySet querySet, uint32_t queryIndex) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuRenderPassEncoderEndPipelineStatisticsQueryWGPU(WGPURenderPassEncoderWGPU renderPassEncoder) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuRenderPassEncoderMultiDrawIndexedIndirectWGPU(WGPURenderPassEncoderWGPU renderPassEncoder, WGPUBuffer indirectBuffer, uint32_t indirectOffset, size_t count) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuRenderPassEncoderMultiDrawIndexedIndirectCountWGPU(WGPURenderPassEncoderWGPU renderPassEncoder, WGPUBuffer indirectBuffer, uint64_t indirectOffset, WGPUBuffer countBuffer, uint64_t countOffset, uint32_t maxCount) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuRenderPassEncoderMultiDrawIndirectWGPU(WGPURenderPassEncoderWGPU renderPassEncoder, WGPUBuffer indirectBuffer, uint32_t indirectOffset, size_t count) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuRenderPassEncoderMultiDrawIndirectCountWGPU(WGPURenderPassEncoderWGPU renderPassEncoder, WGPUBuffer indirectBuffer, uint64_t indirectOffset, WGPUBuffer countBuffer, uint64_t countOffset, uint32_t maxCount) WGPU_FUNCTION_ATTRIBUTE;
WGPU_EXPORT void wgpuRenderPassEncoderSetPushConstantsWGPU(WGPURenderPassEncoderWGPU renderPassEncoder, WGPUShaderStageFlags stages, uint32_t offset, size_t size, void const * data) WGPU_FUNCTION_ATTRIBUTE;

#endif  // !defined(WGPU_SKIP_DECLARATIONS)

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WGPU_H_
