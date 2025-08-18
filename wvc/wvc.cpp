#include <iostream>
#include "wv.h"
#include "wvpp.h"

#define BOOL(b) ((b) ? "true" : "false")

int main()
{
    auto wv = new_wave();
    auto a = wv::NewKnot(wv);
    auto b = wv::NewKnot(wv);
    auto c = wv::NewArrow(wv, a, b);

    std::cout << BOOL(wv::IsArrow(wv, c)) << std::endl;
    std::cout << BOOL(wv::IsMark(wv, c)) << std::endl;

    wv::ChangeSource(wv, c, c);

    std::cout << BOOL(wv::IsArrow(wv, c)) << std::endl;
    std::cout << BOOL(wv::IsMark(wv, c)) << std::endl;

    std::cout << wv::IsNil(wv, c) << std::endl;

    wv::DeleteCascade(wv, &c);
    std::cout << wv::IsNil(wv, c) << std::endl;

    wv::DefineData(wv, "Test", {
        { "i", Datatype::Int },
        { "b", Datatype::Bool },
        { "s", Datatype::String },
        { "f", Datatype::Float },
        { "z", Datatype::String }
    });

    std::cout << BOOL(wv::HasComponent(wv, c, "Test")) << std::endl;

    wv::AddComponent(wv, c, "Test", {
        new int64_t(13),
        new bool(true),
        (void*)"hello",
        new float(3.14f),
        (void*)"world"
    });
    
    std::cout << BOOL(wv::HasComponent(wv, c, "Test")) << std::endl;
    auto t = wv::GetComponent(wv, c, "Test");
    for (auto& [k, v] : t.values) {
        std::cout << "  - " << k << " = (" << (int)v.datatype << ") " << v.value << std::endl;
    }

    auto pi = t.GetInt("i");
    auto pb = t.GetBool("b");
    auto ps = t.GetString("s");
    auto pf = t.GetFloat("f");
    auto pz = t.GetString("z");

    std::cout << pi << " " << pb << " " << ps << " " << pf << " " << pz << std::endl;

    wv::RemoveComponent(wv, c, "Test");
    std::cout << BOOL(wv::HasComponent(wv, c, "Test")) << std::endl;

    auto x = wv::NewKnot(wv);
    auto y = wv::NewKnot(wv);
    wv::NewArrow(wv, x, y);
    wv::NewArrow(wv, y, x);
    auto ars = wv::move::Arrows(wv, { x });
    auto tars = wv::move::Arrows(wv, ars);

    // -------------------------------

    auto hp = wv::NewKnot(wv);
    auto p1 = wv::NewKnot(wv);
    auto p2 = wv::NewKnot(wv);
    auto p3 = wv::NewKnot(wv);
    wv::NewArrow(wv, p1, p2);
    wv::NewArrow(wv, p1, p3);
    wv::NewArrow(wv, p2, p3);
    wv::shape::Hoist(wv, hp, { p1, p2, p3 });

    auto ht = wv::NewKnot(wv);
    auto t1 = wv::NewKnot(wv);
    auto t2 = wv::NewKnot(wv);
    auto t3 = wv::NewKnot(wv);
    auto t4 = wv::NewKnot(wv);
    wv::NewArrow(wv, t1, t2);
    wv::NewArrow(wv, t1, t3);
    wv::NewArrow(wv, t2, t3);
    wv::NewArrow(wv, t3, t2);
    wv::NewArrow(wv, t2, t4);
    wv::NewArrow(wv, t3, t4);
    wv::shape::Hoist(wv, ht, { t1, t2, t3, t4 });

    auto matches = wv::search::FindAll(wv, hp, ht);
    if (matches.has_value())
    {
        for(auto& result : matches.value().entries) 
        {
            for (int i = 0; i < result.count; i++)
            {
                std::cout << " " << i << ": " << result.source[i] << " = " << result.target[i] << std::endl;
            }
            std::cout << "" << std::endl;
        }
    }

    std::cout << "------------" << std::endl;
    auto match = wv::search::FindOne(wv, hp, ht);
    if (match.has_value())
    {
        auto result = match.value();
        for (int i = 0; i < result.count; i++)
        {
            std::cout << " " << i << ": " << result.source[i] << " = " << result.target[i] << std::endl;
        }
    }
}