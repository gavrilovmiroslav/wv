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

namespace weave
{
	class IWeaveLibrary
	{
	public:
		IWeaveLibrary(::Weave* weave) : m_Weave(weave) {}
		virtual ~IWeaveLibrary() {}

	protected:
		::Weave* m_Weave;
	};

	class WeaveLibraryShape : IWeaveLibrary
	{
	public:

		WeaveLibraryShape(::Weave* weave) : IWeaveLibrary(weave)
		{
		}

		void Connect(size_t source, const std::vector<EntityId>& targets)
		{
			wv_shape__connect(m_Weave, source, targets.size(), targets.data());
		}

		void Hoist(size_t subject, const std::vector<EntityId>& objects)
		{
			wv_shape__hoist(m_Weave, subject, objects.size(), objects.data());
		}

		void Lift(const std::vector<EntityId>& arrows)
		{
			wv_shape__lift(m_Weave, arrows.size(), arrows.data());
		}

		void Lower(const std::vector<EntityId>& arrows)
		{
			wv_shape__lower(m_Weave, arrows.size(), arrows.data());
		}

		void Parent(size_t root, const std::vector<EntityId>& children)
		{
			wv_shape__parent(m_Weave, root, children.size(), children.data());
		}

		void Pivot(size_t center, const std::vector<EntityId>& children)
		{
			wv_shape__pivot(m_Weave, center, children.size(), children.data());
		}
	};

	class WeaveLibraryMove : IWeaveLibrary
	{
	public:
		WeaveLibraryMove(::Weave* weave) : IWeaveLibrary(weave)
		{
		}

		std::vector<EntityId> Arrows(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__arrows(m_Weave, it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> ArrowsIn(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__arrows_in(m_Weave, it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> ArrowsOut(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__arrows_out(m_Weave, it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> Deps(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__deps(m_Weave, it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> Down(EntityId it) {
			std::vector<EntityId> result;
			auto arr = wv_move__down(m_Weave, it);
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> DownN(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__down_n(m_Weave, it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> Marks(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__marks(m_Weave, it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> Tethers(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__tethers(m_Weave, it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> Next(EntityId it) {
			std::vector<EntityId> result;
			auto arr = wv_move__next(m_Weave, it);
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> NextN(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__next_n(m_Weave, it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> Prev(EntityId it) {
			std::vector<EntityId> result;
			auto arr = wv_move__prev(m_Weave, it);
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> PrevN(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__prev_n(m_Weave, it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> ToSource(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__to_src(m_Weave, it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> ToTarget(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__to_tgt(m_Weave, it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> Up(EntityId it) {
			std::vector<EntityId> result;
			auto arr = wv_move__up(m_Weave, it);
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}

		std::vector<EntityId> UpN(const std::vector<EntityId>& it) {
			std::vector<EntityId> result;
			auto arr = wv_move__up_n(m_Weave, it.size(), it.data());
			result.assign(arr.ptr, arr.ptr + arr.len);
			return result;
		}
	};

	class WeaveLibrarySearch : IWeaveLibrary
	{
	public:
		WeaveLibrarySearch(::Weave* weave) : IWeaveLibrary(weave)
		{
		}

	public:
		struct SearchResult {
			size_t count;
			std::vector<EntityId> source;
			std::vector<EntityId> target;
		};

		struct SearchResults {
			std::vector<SearchResult> entries;
		};

		std::optional<SearchResult> FindOne(EntityId pattern, EntityId target)
		{
			size_t count{ 0 };
			size_t size{ 0 };
			auto arr = wv_search__find_one(m_Weave, pattern, target, &size, &count);
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

		std::optional<SearchResults> FindAll(EntityId pattern, EntityId target)
		{
			size_t count{ 0 };
			size_t size{ 0 };
			auto arr = wv_search__find_all(m_Weave, pattern, target, &size, &count);
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
	};

	class Weave {
		public:
			Weave()
				: m_Weave{ wv_new_weave() }
			{
			}

			~Weave()
			{
				wv_free_weave(m_Weave);
			}

			inline ::Weave* GetWeave() const
			{
				return m_Weave;
			}

			EntityId NewKnot()
			{
				return wv_new_knot(GetWeave());
			}

			EntityId NewArrow(EntityId src, EntityId tgt)
			{
				return wv_new_arrow(GetWeave(), src, tgt);
			}

			EntityId NewMark(EntityId tgt)
			{
				return wv_new_mark(GetWeave(), tgt);
			}

			EntityId NewTether(EntityId src)
			{
				return wv_new_tether(GetWeave(), src);
			}

			void ChangeSource(EntityId id, EntityId newSrc)
			{
				wv_change_src(GetWeave(), id, newSrc);
			}

			void ChangeTarget(EntityId id, EntityId newTgt)
			{
				wv_change_tgt(GetWeave(), id, newTgt);
			}

			void ChangeEnds(EntityId id, EntityId newSrc, EntityId newTgt)
			{
				wv_change_ends(GetWeave(), id, newSrc, newTgt);
			}

			bool IsNil(EntityId id)
			{
				return wv_is_nil(GetWeave(), id);
			}

			bool IsValid(EntityId id)
			{
				return wv_is_valid(GetWeave(), id);
			}

			bool IsKnot(EntityId id)
			{
				return wv_is_knot(GetWeave(), id);
			}

			bool IsArrow(EntityId id)
			{
				return wv_is_arrow(GetWeave(), id);
			}

			bool IsMark(EntityId id)
			{
				return wv_is_mark(GetWeave(), id);
			}

			bool IsTether(EntityId id)
			{
				return wv_is_tether(GetWeave(), id);
			}

			void DeleteCascade(EntityId* id)
			{
				wv_delete_cascade(GetWeave(), id);
			}

			void DeleteOrphan(EntityId* id)
			{
				wv_delete_orphan(GetWeave(), id);
			}

			bool DefineData(std::string_view name, std::initializer_list<WvDataField> fields)
			{
				return wv_def_data(GetWeave(), name.data(), fields.begin(), fields.size());
			}

			DataId GetDataId(std::string_view name)
			{
				return wv_get_data_id(GetWeave(), name.data());
			}

			size_t GetDataFieldCount(std::string_view name)
			{
				return wv_get_data_field_count(GetWeave(), name.data());
			}

			WvDataField GetDataField(std::string_view name, size_t index)
			{
				return wv_get_data_field(GetWeave(), name.data(), index);
			}

			const void* GetComponentField(EntityId id, std::string_view name, size_t index)
			{
				return wv_get_component_field(GetWeave(), id, name.data(), index);
			}

			void AddComponent(EntityId id, std::string_view name, std::initializer_list<void*> fields)
			{
				wv_add_component(GetWeave(), id, name.data(), fields.begin());
			}

			bool HasComponent(EntityId id, std::string_view name)
			{
				return wv_has_component(GetWeave(), id, name.data());
			}

			void RemoveComponent(EntityId id, std::string_view name)
			{
				return wv_remove_component(GetWeave(), id, name.data());
			}

			DataComponent GetComponent(EntityId id, std::string_view name)
			{
				DataComponent result{};
				const size_t fieldCount = GetDataFieldCount(name);
				for (size_t i = 0; i < fieldCount; i++)
				{
					WvDataField data = GetDataField(name, i);
					DataFieldValue field{};
					field.datatype = data.datatype;
					field.value = GetComponentField(id, name, i);
					result.values[data.name] = field;
				}

				return result;
			}

			WeaveLibraryShape GetShapeLibrary()
			{
				return WeaveLibraryShape(GetWeave());
			}

			WeaveLibraryMove GetMoveLibrary()
			{
				return WeaveLibraryMove(GetWeave());
			}

			WeaveLibrarySearch GetSearchLibrary()
			{
				return WeaveLibrarySearch(GetWeave());
			}

		private:
			::Weave* m_Weave;
		};

	Weave new_weave()
		{
			return Weave();
		}
}