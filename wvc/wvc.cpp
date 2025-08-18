#include <iostream>
#include "wv.h"
#include "wvpp.h"

#define BOOL(b) ((b) ? "true" : "false")

int main()
{
    auto wv = new_weave();
    auto a = wv.NewKnot();
    auto b = wv.NewKnot();
    auto c = wv.NewArrow(a, b);

    std::cout << BOOL(wv.IsArrow(c)) << std::endl;
    std::cout << BOOL(wv.IsMark(c)) << std::endl;

    wv.ChangeSource(c, c);

    std::cout << BOOL(wv.IsArrow(c)) << std::endl;
    std::cout << BOOL(wv.IsMark(c)) << std::endl;

    std::cout << wv.IsNil(c) << std::endl;

    wv.DeleteCascade(&c);
    std::cout << wv.IsNil(c) << std::endl;

    wv.DefineData("Test", {
        { "i", Datatype::Int },
        { "b", Datatype::Bool },
        { "s", Datatype::String },
        { "f", Datatype::Float },
        { "z", Datatype::String }
    });

    std::cout << BOOL(wv.HasComponent(c, "Test")) << std::endl;

    wv.AddComponent(c, "Test", {
        new int64_t(13),
        new bool(true),
        (void*)"hello",
        new float(3.14f),
        (void*)"world"
    });
    
    std::cout << BOOL(wv.HasComponent(c, "Test")) << std::endl;
    auto t = wv.GetComponent(c, "Test");
    for (auto& [k, v] : t.values) {
        std::cout << "  - " << k << " = (" << (int)v.datatype << ") " << v.value << std::endl;
    }

    auto pi = t.GetInt("i");
    auto pb = t.GetBool("b");
    auto ps = t.GetString("s");
    auto pf = t.GetFloat("f");
    auto pz = t.GetString("z");

    std::cout << pi << " " << pb << " " << ps << " " << pf << " " << pz << std::endl;

    wv.RemoveComponent(c, "Test");
    std::cout << BOOL(wv.HasComponent(c, "Test")) << std::endl;

    auto x = wv.NewKnot();
    auto y = wv.NewKnot();
    wv.NewArrow(x, y);
    wv.NewArrow(y, x);
    auto ars = wv.GetMove().Arrows({ x });
    auto tars = wv.GetMove().Arrows(ars);

    // -------------------------------

    auto hp = wv.NewKnot();
    auto p1 = wv.NewKnot();
    auto p2 = wv.NewKnot();
    auto p3 = wv.NewKnot();
    wv.NewArrow(p1, p2);
    wv.NewArrow(p1, p3);
    wv.NewArrow(p2, p3);
    wv.GetShape().Hoist(hp, { p1, p2, p3 });

    auto ht = wv.NewKnot();
    auto t1 = wv.NewKnot();
    auto t2 = wv.NewKnot();
    auto t3 = wv.NewKnot();
    auto t4 = wv.NewKnot();
    wv.NewArrow(t1, t2);
    wv.NewArrow(t1, t3);
    wv.NewArrow(t2, t3);
    wv.NewArrow(t3, t2);
    wv.NewArrow(t2, t4);
    wv.NewArrow(t3, t4);
    wv.GetShape().Hoist(ht, {t1, t2, t3, t4});

    auto matches = wv.GetSearch().FindAll(hp, ht);
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
    auto match = wv.GetSearch().FindOne(hp, ht);
    if (match.has_value())
    {
        auto result = match.value();
        for (int i = 0; i < result.count; i++)
        {
            std::cout << " " << i << ": " << result.source[i] << " = " << result.target[i] << std::endl;
        }
    }
}