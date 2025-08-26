using WeaveCS;

namespace wvcs
{
    using EntityId = uint;

    public class WV
    {
        private readonly IntPtr wv;

        public WV()
        {
            unsafe
            {
                wv = (nint)WVG.wv_new_weave();
            }
        }

        ~WV()
        {
            unsafe
            {
                WVG.wv_free_weave((Weave*)wv);
            }
        }

        public EntityId NewKnot()
        {
            unsafe
            {
                return (uint)WVG.wv_new_knot((Weave*)wv);
            }
        }

        public EntityId NewArrow(EntityId src, EntityId tgt)
        {
            unsafe
            {
                return (uint)WVG.wv_new_arrow((Weave*)wv, src, tgt);
            }
        }

        public EntityId NewMark(EntityId tgt)
        {
            unsafe
            {
                return (uint)WVG.wv_new_mark((Weave*)wv, tgt);
            }
        }

        public EntityId NewTether(EntityId src)
        {
            unsafe
            {
                return (uint)WVG.wv_new_tether((Weave*)wv, src);
            }
        }
    }
}
