#include <iostream>
#include "wv.h"
#include "wvpp.h"

#define BOOL(b) ((b) ? "true" : "false")

int main()
{
    auto a = wv::NewKnot();
    auto b = wv::NewKnot();
    auto c = wv::NewArrow(a, b);

    std::cout << BOOL(wv::IsArrow(c)) << std::endl;
    std::cout << BOOL(wv::IsMark(c)) << std::endl;

    wv::ChangeSource(c, c);

    std::cout << BOOL(wv::IsArrow(c)) << std::endl;
    std::cout << BOOL(wv::IsMark(c)) << std::endl;

    std::cout << wv::IsNil(c) << std::endl;

    wv::DeleteCascade(&c);
    std::cout << wv::IsNil(c) << std::endl;

    wv::DefineData("Test", {
        { "i", Datatype::Int },
        { "b", Datatype::Bool },
        { "s", Datatype::String },
        { "f", Datatype::Float },
        { "z", Datatype::String }
    });

    std::cout << BOOL(wv::HasComponent(c, "Test")) << std::endl;

    wv::AddComponent(c, "Test", {
        new int64_t(13),
        new bool(true),
        (void*)"hello",
        new float(3.14f),
        (void*)"world"
    });
    
    std::cout << BOOL(wv::HasComponent(c, "Test")) << std::endl;
    auto t = wv::GetComponent(c, "Test");
    for (auto& [k, v] : t.values) {
        std::cout << "  - " << k << " = (" << (int)v.datatype << ") " << v.value << std::endl;
    }

    auto pi = t.GetInt("i");
    auto pb = t.GetBool("b");
    auto ps = t.GetString("s");
    auto pf = t.GetFloat("f");
    auto pz = t.GetString("z");

    std::cout << pi << " " << pb << " " << ps << " " << pf << " " << pz << std::endl;

    wv::RemoveComponent(c, "Test");
    std::cout << BOOL(wv::HasComponent(c, "Test")) << std::endl;

    auto x = wv::NewKnot();
    auto y = wv::NewKnot();
    wv::NewArrow(x, y);
    wv::NewArrow(y, x);
    auto ars = wv::move::Arrows({ x });
    auto tars = wv::move::Arrows(ars);
}