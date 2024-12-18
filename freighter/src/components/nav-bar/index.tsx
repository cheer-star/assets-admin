import { Button } from "@/components/ui/button"
import { Link } from "react-router"
import InteractiveHoverButton from "../ui/interactive-hover-button"
import { GithubIcon } from "../icons"

export default function Home() {
  return (
    <div className="px-8 py-4 flex justify-between sticky top-0 backdrop-blur-2xl ">
      <div className="flex">
        <div>
          <img src="/logo.png" className="size-10 inline-block" />
          <span className="inline-block font-bold ml-2 relative top-[4px] text-lg">资产管理系统</span>
        </div>

      </div>
      <div className="flex">
        <Link className="relative top-[9px] mr-8" to="/">文档</Link>
        <Link className="relative top-[9px] mr-8" to="/login">关于</Link>

        <Button variant="ghost" size="icon" className="mr-8">
          <GithubIcon />
        </Button>

        <InteractiveHoverButton text="演示" className="h-10"></InteractiveHoverButton>
      </div>
    </div>
  )
}
