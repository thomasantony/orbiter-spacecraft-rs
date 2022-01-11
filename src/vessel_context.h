#pragma once
// DO NOT IMPORT lib.rs.h here
#include "rust/cxx.h"
#include "orbitersdk.h"
#include "box_dyn_vessel.h"

void debugLog(rust::Str);

using Vector3 = VECTOR3;

// Wrapper for oapiCreateVessel
OBJHANDLE oapi_create_vessel(rust::String name, rust::String classname, const VESSELSTATUS& status);

// ==============================================================
// Spacecraft class interface
// ==============================================================
class VesselContext : public VESSEL4
{
public:
    VesselContext(OBJHANDLE hVessel, int flightmodel);
    ~VesselContext();
    void clbkSetClassCaps(FILEHANDLE cfg);
    void clbkPreStep(double SimT, double SimDT, double MJD);
    int clbkConsumeBufferedKey(DWORD key, bool down, char *kstate);

    void AddMesh(rust::String mesh_name) const;
    void AddMeshWithOffset(rust::String mesh_name, const Vector3& ofs) const;
    size_t AddExhaust(THRUSTER_HANDLE th, double lscale, double wscale) const;

    THRUSTER_HANDLE CreateThruster(const Vector3 &pos, const Vector3 &dir, double maxth0, PROPELLANT_HANDLE ph, double isp) const;
    PROPELLANT_HANDLE CreatePropellantResource(double mass) const;
    THGROUP_HANDLE CreateThrusterGroup(rust::Slice<const THRUSTER_HANDLE> thrusters, THGROUP_TYPE thgroup_type) const;

    rust::Str GetName() const;
    double GetThrusterGroupLevelByType(THGROUP_TYPE thgroup_type) const;
private:
    BoxDynVessel rust_spacecraft_;
};