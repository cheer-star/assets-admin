import Particles from "../ui/particles";

export default function Body() {

    return (
        <div className="relative flex h-[500px] w-full flex-col items-center justify-center overflow-hidden rounded-lg  bg-background">
            <span className="pointer-events-none whitespace-pre-wrap bg-gradient-to-b from-black to-gray-300/80 bg-clip-text text-center text-6xl font-semibold leading-none text-transparent dark:from-white dark:to-slate-900/10">
                快速、简洁
            </span>
            <span className="mt-4 pointer-events-none whitespace-pre-wrap bg-gradient-to-b from-black to-gray-300/80 bg-clip-text text-center text-6xl font-semibold leading-none text-transparent dark:from-white dark:to-slate-900/10">
                随意组合功能
            </span>
            <Particles
                className="absolute inset-0"
                quantity={100}
                ease={80}
                color="#000000"
                refresh
            />
        </div>
    )
}