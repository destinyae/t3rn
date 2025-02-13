import { SubmittableExtrinsic } from "@polkadot/api/promise/types"
import * as fs from "fs"
import * as process from "process"
import * as path from "path"

require("dotenv").config()

interface ExtrinsicParam {
  name: string
  rustType: string
  encoded: string
  decoded: any
}

interface EventParam {
  section: string
  method: string
  encoded: string
  decoded: any
}

export class ExtrinsicExport {
  tx: SubmittableExtrinsic
  section: string
  method: string
  args: ExtrinsicParam[] = []
  events: EventParam[] = []
  submissionHeight: number
  signer: string
  error: string = ""

  constructor(tx: SubmittableExtrinsic, address: string) {
    this.tx = tx
    this.signer = address
    this.handleParams()
  }

  handleParams() {
    const decoded = this.tx.method.toHuman(true)
    // @ts-ignore
    this.section = decoded.section.toString()
    // @ts-ignore
    this.method = decoded.method.toString()

    const args = this.tx.method.args
    // @ts-ignore
    const paramDesc = this.tx.method._meta.toJSON().args
    //
    for (let i = 0; i < args.length; i++) {
      const param: ExtrinsicParam = {
        name: paramDesc[i].name,
        rustType: paramDesc[i].typeName,
        encoded: args[i].toHex().substring(2), // remove 0x prefix
        // @ts-ignore - TS doesn't know about toPrimitive
        decoded: JSON.stringify(args[i].toPrimitive()),
      }
      this.args.push(param)
    }
  }

  addEvent(event: any) {
    const decoded = event.toHuman(true)
    const eventType: EventParam = {
      section: decoded.section,
      method: decoded.method,
      encoded: event.toHex().substring(2), // remove 0x prefix
      decoded: JSON.stringify(decoded),
    }

    this.events.push(eventType)
  }

  public addErr(dispatchError: any) {
    this.error = dispatchError.toHex().substring(2)
    return this
  }

  public addSubmissionHeight(height: number) {
    this.submissionHeight = height
    return this
  }

  toJSON() {
    return JSON.stringify(
      {
        section: this.section,
        method: this.method,
        args: this.args,
        submissionHeight: this.submissionHeight,
        signer: this.signer,
        error: this.error,
        events: this.events,
      },
      null,
      4,
    )
  }

  toFile() {
    const dir = process.env.EXPORT_PATH || "./exports"
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir)
    }

    const fileName = `${Date.now()}_${this.section}_${this.method}.json`
    fs.writeFileSync(path.join(dir, fileName), this.toJSON())
  }
}
