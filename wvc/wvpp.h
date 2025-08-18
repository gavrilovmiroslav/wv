#pragma once

#include "wv.h"
#include <cassert>
#include <map>
#include <memory>
#include <optional>
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
public:
	WV()
		: m_Weave{ wv_new_weave() }
	{
	}

	~WV()
	{
		wv_free_weave(m_Weave);
	}

	inline Weave* GetWeave() const
	{
		return m_Weave;
	}

private:
	Weave* m_Weave;
};

WV new_wave()
{
	return WV();
}

namespace wv 
{
	static EntityId NewKnot(const WV& wv)
	{
		return wv_new_knot(wv.GetWeave());
	}

	static EntityId NewArrow(const WV& wv, EntityId src, EntityId tgt)
	{
		return wv_new_arrow(wv.GetWeave(), src, tgt);
	}

	static EntityId NewMark(const WV& wv, EntityId tgt)
	{
		return wv_new_mark(wv.GetWeave(), tgt);
	}

	static EntityId NewTether(const WV& wv, EntityId src)
	{
		return wv_new_tether(wv.GetWeave(), src);
	}

	static void ChangeSource(const WV& wv, EntityId id, EntityId newSrc)
	{
		wv_change_src(wv.GetWeave(), id, newSrc);
	}

	static void ChangeTarget(const WV& wv, EntityId id, EntityId newTgt)
	{
		wv_change_tgt(wv.GetWeave(), id, newTgt);
	}

	static void ChangeEnds(const WV& wv, EntityId id, EntityId newSrc, EntityId newTgt)
	{
		wv_change_ends(wv.GetWeave(), id, newSrc, newTgt);
	}

	static bool IsNil(const WV& wv, EntityId id)
	{
		return wv_is_nil(wv.GetWeave(), id);
	}

	static bool IsValid(const WV& wv, EntityId id)
	{
		return wv_is_valid(wv.GetWeave(), id);
	}

	static bool IsKnot(const WV& wv, EntityId id)
	{
		return wv_is_knot(wv.GetWeave(), id);
	}

	static bool IsArrow(const WV& wv, EntityId id)
	{
		return wv_is_arrow(wv.GetWeave(), id);
	}

	static bool IsMark(const WV& wv, EntityId id)
	{
		return wv_is_mark(wv.GetWeave(), id);
	}

	static bool IsTether(const WV& wv, EntityId id)
	{
		return wv_is_tether(wv.GetWeave(), id);
	}

	static void DeleteCascade(const WV& wv, EntityId* id)
	{
		wv_delete_cascade(wv.GetWeave(), id);
	}

	static void DeleteOrphan(const WV& wv, EntityId* id)
	{
		wv_delete_orphan(wv.GetWeave(), id);
	}

	static bool DefineData(const WV& wv, std::string_view name, std::initializer_list<WvDataField> fields)
	{
		return wv_def_data(wv.GetWeave(), name.data(), fields.begin(), fields.size());
	}

	static DataId GetDataId(const WV& wv, std::string_view name)
	{
		return wv_get_data_id(wv.GetWeave(), name.data());
	}

	static size_t GetDataFieldCount(const WV& wv, std::string_view name)
	{
		return wv_get_data_field_count(wv.GetWeave(), name.data());
	}

	static WvDataField GetDataField(const WV& wv, std::string_view name, size_t index)
	{
		return wv_get_data_field(wv.GetWeave(), name.data(), index);
	}

	static const void* GetComponentField(const WV& wv, EntityId id, std::string_view name, size_t index)
	{
		return wv_get_component_field(wv.GetWeave(), id, name.data(), index);
	}

	static void AddComponent(const WV& wv, EntityId id, std::string_view name, std::initializer_list<void*> fields)
	{
		wv_add_component(wv.GetWeave(), id, name.data(), fields.begin());
	}

	static bool HasComponent(const WV& wv, EntityId id, std::string_view name)
	{
		return wv_has_component(wv.GetWeave(), id, name.data());
	}

	static void RemoveComponent(const WV& wv, EntityId id, std::string_view name)
	{
		return wv_remove_component(wv.GetWeave(), id, name.data());
	}

	static DataComponent GetComponent(const WV& wv, EntityId id, std::string_view name)
	{
		DataComponent result{};
		const size_t fieldCount = GetDataFieldCount(wv, name);
		for (size_t i = 0; i < fieldCount; i++)
		{
			WvDataField data = GetDataField(wv, name, i);
			DataFieldValue field{};
			field.datatype = data.datatype;
			field.value = GetComponentField(wv, id, name, i);
			result.values[data.name] = field;
		}

		return result;
	}

	namespace shape 
	{
		static void Connect(const WV& wv, size_t source, const std::vector<EntityId>& targets)
		{
			wv_shape__connect(wv.GetWeave(), source, targets.size(), targets.data());
		}

		static void Hoist(const WV& wv, size_t subject, const std::vector<EntityId>& objects)
		{
			wv_shape__hoist(wv.GetWeave(), subject, objects.size(), objects.data());
		}

		static void Lift(const WV& wv, const std::vector<EntityId>& arrows)
		{
			wv_shape__lift(wv.GetWeave(), arrows.size(), arrows.data());
		}

		static void Lower(const WV& wv, const std::vector<EntityId>& arrows)
		{
			wv_shape__lower(wv.GetWeave(), arrows.size(), arrows.data());
		}

		static void Parent(const WV& wv, size_t root, const std::vector<EntityId>& children)
		{
			wv_shape__parent(wv.GetWeave(), root, children.size(), children.data());
		}

		static void Pivot(const WV& wv, size_t center, const std::vector<EntityId>& children)
		{
			wv_shape__pivot(wv.GetWeave(), center, children.size(), children.data());
		}
	}

	namespace move {
		static std::vector<EntityId> Arrows(const WV& wv, const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__arrows(wv.GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		static std::vector<EntityId> ArrowsIn(const WV& wv, const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__arrows_in(wv.GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		static std::vector<EntityId> ArrowsOut(const WV& wv, const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__arrows_out(wv.GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		static std::vector<EntityId> Deps(const WV& wv, const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__deps(wv.GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		static std::vector<EntityId> Down(const WV& wv, EntityId it) {
			std::vector<EntityId> result;
			auto arr = wv_move__down(wv.GetWeave(), it);
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		static std::vector<EntityId> DownN(const WV& wv, const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__down_n(wv.GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		static std::vector<EntityId> Marks(const WV& wv, const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__marks(wv.GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		static std::vector<EntityId> Tethers(const WV& wv, const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__tethers(wv.GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		static std::vector<EntityId> Next(const WV& wv, EntityId it) {
			std::vector<EntityId> result;
			auto arr = wv_move__next(wv.GetWeave(), it);
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		static std::vector<EntityId> NextN(const WV& wv, const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__next_n(wv.GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		static std::vector<EntityId> Prev(const WV& wv, EntityId it) {
			std::vector<EntityId> result;
			auto arr = wv_move__prev(wv.GetWeave(), it);
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		static std::vector<EntityId> PrevN(const WV& wv, const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__prev_n(wv.GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		static std::vector<EntityId> ToSource(const WV& wv, const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__to_src(wv.GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		static std::vector<EntityId> ToTarget(const WV& wv, const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__to_tgt(wv.GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		static std::vector<EntityId> Up(const WV& wv, EntityId it) {
			std::vector<EntityId> result;
			auto arr = wv_move__up(wv.GetWeave(), it);
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		static std::vector<EntityId> UpN(const WV& wv, const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__up_n(wv.GetWeave(), it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}
	}

	namespace search {
		struct SearchResult {
			size_t count;
			std::vector<EntityId> source;
			std::vector<EntityId> target;
		};

		struct SearchResults {
			std::vector<SearchResult> entries;
		};

		static std::optional<SearchResult> FindOne(const WV& wv, EntityId pattern, EntityId target)
		{
			size_t count{ 0 };
			size_t size{ 0 };
			auto arr = wv_search__find_one(wv.GetWeave(), pattern, target, &size, &count);
			if (count == 0) return std::nullopt;

			std::vector<EntityId> unwrapped;
			unwrapped.assign(arr.ptr, arr.ptr + arr.len);

			SearchResult result;
			result.count = size;
			for (int i = 0; i < size * 2; i += 2)
			{
				result.source.push_back(unwrapped[i]);
				result.target.push_back(unwrapped[i + 1]);
			}

			return std::optional(result);
		}

		static std::optional<SearchResults> FindAll(const WV& wv, EntityId pattern, EntityId target)
		{
			size_t count{ 0 };
			size_t size{ 0 };
			auto arr = wv_search__find_all(wv.GetWeave(), pattern, target, &size, &count);
			if (count == 0) return std::nullopt;

			std::vector<EntityId> unwrapped;
			unwrapped.assign(arr.ptr, arr.ptr + arr.len);

			SearchResults results;

			size_t n = 0;
			for (int i = 0; i < count; i++)
			{
				SearchResult result;
				result.count = size;
				for (int j = 0; j < size; j++)
				{
					result.source.push_back(unwrapped[n++]);
					result.target.push_back(unwrapped[n++]);
				}

				results.entries.push_back(std::move(result));
			}

			return std::optional(results);
		}
	}
};