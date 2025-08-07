#pragma once

#include "wv.h"
#include <cassert>
#include <map>
#include <memory>
#include <vector>

using EntityId = size_t;
using DataId = uint64_t;

struct DataFieldValue {
	Datatype datatype;
	const void* value;
};

struct DataComponent {
	std::map<std::string_view, DataFieldValue> values;

	int32_t GetInt(std::string_view name)
	{
		DataFieldValue& value = values[name];
		assert(value.datatype == Datatype::Int);

		return *(int32_t*)value.value;
	}

	float GetFloat(std::string_view name)
	{
		DataFieldValue& value = values[name];
		assert(value.datatype == Datatype::Float);

		return *(float*)value.value;
	}

	bool GetBool(std::string_view name)
	{
		DataFieldValue& value = values[name];
		assert(value.datatype == Datatype::Bool);

		return *(bool*)value.value;
	}

	std::string_view GetString(std::string_view name)
	{
		DataFieldValue& value = values[name];
		assert(value.datatype == Datatype::String);

		return (const char*)value.value;
	}
};

class WV {
	Weave* m_Weave;

	WV()
		: m_Weave{ wv_new_weave() }
	{
	}

	~WV()
	{
		wv_free_weave(m_Weave);
	}

public:
	WV(WV const&) = delete;
	void operator=(WV const&) = delete;

	static WV& Get()
	{
		static WV ms_Instance;
		return ms_Instance;
	}

	static Weave* GetWeave()
	{
		return WV::Get().m_Weave;
	}
};

namespace wv 
{
	static EntityId NewKnot()
	{
		return wv_new_knot(WV::GetWeave());
	}

	static EntityId NewArrow(EntityId src, EntityId tgt)
	{
		return wv_new_arrow(WV::GetWeave(), src, tgt);
	}

	static EntityId NewMark(EntityId tgt)
	{
		return wv_new_mark(WV::GetWeave(), tgt);
	}

	static EntityId NewTether(EntityId src)
	{
		return wv_new_tether(WV::GetWeave(), src);
	}

	static void ChangeSource(EntityId id, EntityId newSrc)
	{
		wv_change_src(WV::GetWeave(), id, newSrc);
	}

	static void ChangeTarget(EntityId id, EntityId newTgt)
	{
		wv_change_tgt(WV::GetWeave(), id, newTgt);
	}

	static void ChangeEnds(EntityId id, EntityId newSrc, EntityId newTgt)
	{
		wv_change_ends(WV::GetWeave(), id, newSrc, newTgt);
	}

	static bool IsNil(EntityId id)
	{
		return wv_is_nil(WV::GetWeave(), id);
	}

	static bool IsValid(EntityId id)
	{
		return wv_is_valid(WV::GetWeave(), id);
	}

	static bool IsKnot(EntityId id)
	{
		return wv_is_knot(WV::GetWeave(), id);
	}

	static bool IsArrow(EntityId id)
	{
		return wv_is_arrow(WV::GetWeave(), id);
	}

	static bool IsMark(EntityId id)
	{
		return wv_is_mark(WV::GetWeave(), id);
	}

	static bool IsTether(EntityId id)
	{
		return wv_is_tether(WV::GetWeave(), id);
	}

	static void DeleteCascade(EntityId* id)
	{
		wv_delete_cascade(WV::GetWeave(), id);
	}

	static void DeleteOrphan(EntityId* id)
	{
		wv_delete_orphan(WV::GetWeave(), id);
	}

	static bool DefineData(std::string_view name, std::initializer_list<WvDataField> fields)
	{
		return wv_def_data(WV::GetWeave(), name.data(), fields.begin(), fields.size());
	}

	static DataId GetDataId(std::string_view name)
	{
		return wv_get_data_id(WV::GetWeave(), name.data());
	}

	static size_t GetDataFieldCount(std::string_view name)
	{
		return wv_get_data_field_count(WV::GetWeave(), name.data());
	}

	static WvDataField GetDataField(std::string_view name, size_t index)
	{
		return wv_get_data_field(WV::GetWeave(), name.data(), index);
	}

	static const void* GetComponentField(EntityId id, std::string_view name, size_t index)
	{
		return wv_get_component_field(WV::GetWeave(), id, name.data(), index);
	}

	static void AddComponent(EntityId id, std::string_view name, std::initializer_list<void*> fields)
	{
		wv_add_component(WV::GetWeave(), id, name.data(), fields.begin());
	}

	static bool HasComponent(EntityId id, std::string_view name)
	{
		return wv_has_component(WV::GetWeave(), id, name.data());
	}

	static void RemoveComponent(EntityId id, std::string_view name)
	{
		return wv_remove_component(WV::GetWeave(), id, name.data());
	}

	static DataComponent GetComponent(EntityId id, std::string_view name)
	{
		DataComponent result{};
		const int fieldCount = GetDataFieldCount(name);
		for (int i = 0; i < fieldCount; i++)
		{
			WvDataField data = GetDataField(name, i);
			DataFieldValue field{};
			field.datatype = data.datatype;
			field.value = GetComponentField(id, name, i);
			result.values[data.name] = field;
		}

		return result;
	}

	namespace shape 
	{
		void Connect(size_t source, const std::vector<EntityId>& targets)
		{
			wv_shape__connect(WV::GetWeave(), source, targets.size(), targets.data());
		}

		void Hoist(size_t subject, const std::vector<EntityId>& objects)
		{
			wv_shape__connect(WV::GetWeave(), subject, objects.size(), objects.data());
		}

		void Lift(const std::vector<EntityId>& arrows)
		{
			wv_shape__lift(WV::GetWeave(), arrows.size(), arrows.data());
		}

		void Lower(const std::vector<EntityId>& arrows)
		{
			wv_shape__lower(WV::GetWeave(), arrows.size(), arrows.data());
		}

		void Parent(size_t root, const std::vector<EntityId>& children)
		{
			wv_shape__parent(WV::GetWeave(), root, children.size(), children.data());
		}

		void Pivot(size_t center, const std::vector<EntityId>& children)
		{
			wv_shape__pivot(WV::GetWeave(), center, children.size(), children.data());
		}
	}

	namespace move {
		std::vector<EntityId> Arrows(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__arrows(WV::GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> ArrowsIn(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__arrows_in(WV::GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> ArrowsOut(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__arrows_out(WV::GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> Deps(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__deps(WV::GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> Down(EntityId it) {
			std::vector<EntityId> result;
			auto arr = wv_move__down(WV::GetWeave(), it);
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> DownN(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__down_n(WV::GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> Marks(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__marks(WV::GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> Tethers(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__tethers(WV::GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> Next(EntityId it) {
			std::vector<EntityId> result;
			auto arr = wv_move__next(WV::GetWeave(), it);
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> NextN(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__next_n(WV::GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> Prev(EntityId it) {
			std::vector<EntityId> result;
			auto arr = wv_move__prev(WV::GetWeave(), it);
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> PrevN(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__prev_n(WV::GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> ToSource(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__to_src(WV::GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> ToTarget(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__to_tgt(WV::GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> Up(EntityId it) {
			std::vector<EntityId> result;
			auto arr = wv_move__up(WV::GetWeave(), it);
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> UpN(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__up_n(WV::GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}
	}
};