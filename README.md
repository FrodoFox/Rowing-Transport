# Rowing Transport Manager

A desktop application for managing transport logistics for rowing club events. Assign squad members to boats, then automatically allocate drivers and vehicles to get everyone to the venue — and generate a printable PDF transport manifest.

---

## Features

- Drag-and-drop boat builder with support for singles through to coxed eights
- Automatic transport allocation across minibuses and personal cars
- Gender-balanced passenger assignment with location and seniority prioritisation
- "Wants to Drive" opt-in for members with their own car
- PDF transport manifest generation
- Persistent squad and minibus data via local JSON files

---

## Prerequisites

You will need the following installed before building:

- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain, 1.70+)

That's it. All dependencies are managed by Cargo and will be downloaded automatically on first build.

---

## Installation

**1. Download Rust**

Open the terminal and run (or download manually from website):
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**2. Clone the repository**

Open the terminal and run:
```bash
cd YOU/PREFERRED/DOWNLOAD/LOCATION
git clone https://github.com/FrodoFox/Transport-Automation.git
cd Transport-Automation
```

## Building and running

**1. Build the Project**

This simply builds the files, installs all dependencies within the code and allows it to run faster in future (essentially building its own work environment).

```bash
cd YOU/PREFERRED/DOWNLOAD/LOCATION/Transport-Automation
cargo build
```

The first build will take a few minutes while Cargo downloads and compiles dependencies. Subsequent builds will be much faster.

**2. Run the Project -- Reuse this part**

In the same directory you ran **build** you now need to run:

```bash
cd YOU/PREFERRED/DOWNLOAD/LOCATION/Transport-Automation
cargo run --version
```

---

## Data Files

The app reads and writes two JSON files in the **same directory you run it from**:

| File | Contents |
|---|---|
| `people.json` | Squad members and their details |
| `minibuses.json` | Club minibuses and seat counts |

These files are created automatically when you first add people or minibuses through the app. You can also create them manually — see the structure below.

### `people.json` example

```json
[
  {
    "name": "Alice Smith",
    "gender": "Female",
    "student_id": "s1234567",
    "year_of_entry": 2022,
    "pickup_locations": ["Pleasance"],
    "can_drive_minibus": false,
    "car": {
      "vehicle_type": "Hatchback",
      "registration": "AB12 CDE",
      "seats": 5
    }
  }
]
```

Set `"car"` to `null` if the person does not own a car.

### `minibuses.json` example

```json
[
  {
    "registration": "SG21 ABC",
    "seats": 16
  }
]
```

---

## Usage

1. **Add your squad** — use the *＋ Add Person* button in the sidebar, or populate `people.json` directly.
2. **Add minibuses** — edit `minibuses.json` or use the Edit button in the Minibuses section.
3. **Build your crews** — click boat type buttons to add boats to the canvas, then select a person from the sidebar and click a seat to assign them.
4. **Set departure times and destinations** — each boat has a time input and a destination dropdown above it.
5. **Publish** — click *Publish & PDF* to run the transport allocation and generate `transport_manifest.pdf` in the current directory.

### Stipulations

1. **Economic Target** — This app sets an economic target by being most efficient in transporting people, it doesn't care who drives or how often. It focusses on getting everyone from A to B with a minimal cars required as possible. This can be tweaked on request but given the nature of the finance of the club I thought best to design it that way.
2. **Private Transport** — People wishing to be removed from the journey need only let the driver know. The transport is derived from the crew list. Meaning to update the transport you'd need to update the crews. So it's easier if they simply let the driver know in advance and are ignored on transport that day. Again this could be edited in a later update to allow tweaking the transport sheet in terms of who goes when after it's been initially formed.
3. **GDPR Privacy** — This app runs off a lot of data. That data is used in managing who is sat where in what car to best manage squad integration and helping people get along (I thought it was a nice idea). This app doesn't require anymore information than the DVLA and university already has. Cross referencing the two for either party would be simple. If a person is not willing to give up their data as they feel it may be invasive or violate their privacy in some way then they are under no obligation to do so and this app should not be used as a method for trying to wager them out of information. Any information is stored locally on a host computer, off the internet and is deleted when that instance inside the app is deleted. In compliance with GDPR.

---

## Troubleshooting

**The app won't start**
Make sure you are running `cargo run` from the project root directory (where `Cargo.toml` lives).

**My data isn't saving**
The app writes `people.json` and `minibuses.json` to whatever directory your terminal is in when you run it. Make sure you always launch it from the same location.

**PDF isn't generating**
Check the error dialog — it will report the reason. Ensure all boats have a departure time, a destination selected, and every seat filled before publishing.

---
