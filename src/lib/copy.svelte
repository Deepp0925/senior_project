<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"
  import { listen } from "@tauri-apps/api/event"

  let src = "../testing/bike.blend1";
  let dst = "/Volumes/PNY 2/test_dst";

  let progress = 0;
  let processed = 0;
  let total = 222123236;
  let time = 0;
  let completed = false;

  let start_timer = performance.now();

  let logs = [
    "Logs will be displayed here",
  ];

  listen("progress", (msg) => {
    progress = msg.payload as number;
  });

  listen("processed", (evt) => {
    processed += evt.payload as number;
    progress = (processed/total)*100;

    if (progress >= 100) {
        completed = true;
        progress = 100;
        time = performance.now() - start_timer;
    }
  });

  listen("worker-done" , async (evt) => {
    let res = await invoke("completed_worker", {id: evt.payload}) as boolean;
    if (!res) {
        let completed = await invoke("is_complete") as boolean;
        if (completed) {
            completed = true;
            progress = 100;
        }
    }
  })

  listen("log", (evt) => {
      logs.push(evt.payload as string);
  });

    async function start(){
        let res = await invoke("init", {src, dst});
        console.log(res);
        res = await invoke("start") as boolean;
        console.log(res);
    }

    start().then(() => {
        console.log("started");
    });

//   function set_src(e){
//     e.preventDefault();
//     src = e.dataTransfer.files[0].path;
//   }

//     function set_dst(e){
//         e.preventDefault();
//         dst = e.dataTransfer.files[0].path;
//     }


</script>


<!-- 

    <div class="flex flex-row h-full w-full px-8 items-center" >
    <div class="w-1/2 mr-2 h-96 hover:border-sky-700 hover:border-2 hover:border-dashed hover:transition" on:dragover|preventDefault on:drop|preventDefault={set_src}  >
        <div class="flex flex-col h-full justify-center items-center">
            <i data-feather="file" class="mb-10 text-lg"></i>
            
            <input id="src-input" class="bg-gray-100 p-2 rounded-md w-3/5" placeholder="Enter a source path..." bind:value={src} />
        </div>
    </div>
    
    <div class="w-1/2  ml-2 h-96">
      <div class="flex flex-col h-full justify-center items-center">
        <i data-feather="map-pin" class="mb-10 text-lg"></i>
        
        <input id="dst-input" class="bg-gray-100 p-2 rounded-md w-3/5" placeholder="Enter a destination path..." bind:value={dst} />
        </div>
    </div>
  </div>
 -->


<div class="flex flex-col flex-1 items-center">
    <!-- half is dedicated to progress and the other is dedicated for logs -->
    <div class="flex-1 w-11/12 items-center flex flex-col">
       <div class="indeterminate-progress-bar h-5 w-full">
            <div class="indeterminate-progress-bar__progress" style="width: {progress}%;"></div>
        </div> 
        <div class="flex-1 flex flex-row w-full">
          <div class="flex-1 flex-col">
            <div class="flex flex-col mt-3">
              <div class="flex-1 flex w-full text-sm">Source:</div>
              <div class="flex-1 flex w-full text-md font-semibold">{src}</div>
            </div>
            <div class="flex flex-col mt-3">
              <div class="flex-1 flex w-full text-sm">Destination:</div>
              <div class="flex-1 flex w-full text-md font-semibold">{dst}</div>
            </div>
            <div class="flex flex-col mt-3">
              <div class="flex-1 flex w-full text-sm">Processed:</div>
              <div class="flex-1 flex w-full text-md font-semibold">{processed}</div>
            </div>

            <div class="flex flex-col mt-3">
              <div class="flex-1 flex w-full text-sm">Time taken:</div>
              <div class="flex-1 flex w-full text-md font-semibold">{time/1000} seconds</div>
            </div>
            
          </div>
          <div class="flex-1 flex w-full justify-end text-5xl">{Math.round(progress)}%</div>
        </div>
    </div>
    <div class="class logs flex-1 w-11/12 m-2 bg-gray-500 overflow-scroll">
        <!-- render all the logs -->
        
        <ul class="p-3">
          {#each logs as log}
            <li class="mb-2 text-white">{log}</li>
          {/each}
        </ul>

    </div>
    
</div>