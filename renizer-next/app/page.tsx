"use client";

import { NavigationMenuComp as NavigationMenu } from "@/components/ui/home/navigation-menu";
import RadialGradient from "@/components/ui/magicui/radial-gradient";
import RetroGrid from "@/components/ui/magicui/retro-grid";
import LoginForm from "@/components/ui/home/login-form";

export default function Home() {
  return (
    <div className="grid place-items-center">
      <NavigationMenu title="New Here?" />
      <RadialGradient className="w-full h-[40rem] p-20 mt-10 grid place-items-center">
        <h1 className="text-balance bg-gradient-to-br from-black from-30% to-black/60 bg-clip-text py-6 text-5xl font-semibold leading-none tracking-tighter text-transparent dark:from-white dark:to-white/40 sm:text-4xl md:text-7xl lg:text-7xl text-center">Join the Movement Towards Sustainable Energy Solutions with REnizer
        </h1>
        <LoginForm />
        <RetroGrid />
      </RadialGradient>
    </div>
  );
}
