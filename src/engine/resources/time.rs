// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use sfml::system::Time;

pub(crate) fn add_default_resources(world: &mut ::specs::World) {
    world.add_resource(DeltaTime::default());
    world.add_resource(Elapsed::default());
    world.add_resource(PhysicsDeltaTime::default());
    world.add_resource(PhysicsElapsed::default());
}

/// The time since the last update/fixed update.
pub struct DeltaTime(pub Time);

impl Default for DeltaTime {
    fn default() -> Self { DeltaTime(Time::ZERO) }
}

/// The total elapsed time since the simulation started.
pub struct Elapsed {
    pub previous: Time,
    pub current: Time,
}

impl Default for Elapsed {
    fn default() -> Self { 
        Elapsed {
            current: Time::ZERO,
            previous: Time::ZERO,
        }
    }
}

/// The amount of time elapsed in the physics simulation.
pub struct PhysicsDeltaTime(pub Time);

impl Default for PhysicsDeltaTime {
    fn default() -> Self { PhysicsDeltaTime(Time::milliseconds(16)) }
}

/// The total elapsed time in the physics simulation.
pub struct PhysicsElapsed {
    pub previous: Time,
    pub current: Time,
}

impl Default for PhysicsElapsed {
    fn default() -> Self { 
        PhysicsElapsed {
            current: Time::ZERO,
            previous: Time::ZERO,
        }
    }
}

/// Calculate an interpolation factor from the previous physics timestep to now.
pub fn physics_interpolation_factor(elapsed: &Elapsed, physics: &PhysicsElapsed) -> f32 {
    let since_physics = elapsed.current - physics.previous;
    let physics_step = physics.current - physics.previous;
    since_physics.as_seconds() / physics_step.as_seconds()
}


