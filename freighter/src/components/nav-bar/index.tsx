import { Button } from "@/components/ui/button"

export default function Home() {
  return (
    <div className="flex justify-between px-12 py-4 sticky top-0 backdrop-blur-2xl  border-b border-slate-900/10">
      <div className="flex">
        <div className="h-6">
          <img src="/logo.png" className="size-6 inline-block m-1" />
          <span className="inline-block relative top-[3px] font-bold">资产管理系统</span>
        </div>
        <div className="px-8">nav1</div>
        <div className="px-8">nav2</div>
        <div className="px-8">nav3</div>
      </div>
      <div>option</div>
    </div>
  )
}
