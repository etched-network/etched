
import * as process from 'process'
// import dotenv
import 'dotenv/config'

export class Settings {
    public static get (setting: string): string {
        if (!process.env[setting]) {
            throw new Error(`ENV: ${setting} not configured`)
        }
        return process.env[setting] as string
    }
}
