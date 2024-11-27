// import Login from "@/components/Login"

import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle
} from '@/components/ui/card'

import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'

import './index.css'

/**
 * 页面布局：
 * * 期望： https://pro.magicui.design/login
 * 1. 使用 Card 组件作为基座
 * 2. 使用表单提供登陆
 */
export default function LoginPage() {
    return (
        <div className='container'>
            <Card className='w-[450px]'>
                <CardHeader>
                    <CardTitle>登陆系统</CardTitle>
                    <CardDescription>输入你的账号密码登陆资产管理系统.</CardDescription>
                </CardHeader>
                <CardContent>
                    {/* 这部分是登陆表单 */}
                    <Input type='text' placeholder='账号' />
                    <br />
                    <Input type='password' placeholder='密码' />
                </CardContent>
                <CardFooter>
                    <Button>登陆</Button>
                    <Button variant="link">没有账号？联系管理员</Button>
                </CardFooter>
            </Card>
        </div>
        // <Login />
    )
}
