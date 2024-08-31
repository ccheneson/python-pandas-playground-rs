<template>
  <div id="form">
      <div id="inputs">
        <label>API name</label>
        <input v-model="input.api" type="text" placeholder="API name" name="api" required />   
        <div id="submitblock">
          <button class="submit" @click="submit">Submit</button>
          <img  v-if="isSubmit" src="@/assets/icons8-spinner.gif"/>
        </div>
        <label class="errors" v-if="errors.api">{{ errors.api }}</label>
      </div>

 
   
      <div>
        <label >Output</label>
        <textarea v-model="output.response"  rows="30" cols="100" class="outputarea" />
      </div>
  
  </div>
</template>


<script lang="ts">

interface FormErrors {
  api: string; 
  code: string;
}

export default {
  data() {
    return {
      errors: {
        api: "",
        code: ""
      },
      input: {
        api: "",
        code : ""
      },
      output: {
        response: '',  
      },
      isSubmit: false
    }
  },
  methods: {
    validate() {
      if (this.input.api == "") {
        this.errors.api = "API name required"
      }
      return Object.keys(this.errors).length
    },
    submit() {
      this.errors = {};
      this.isSubmit = false;

      if (this.validate()) {
        return false
      } else {
      this.isSubmit = true;
      fetch(`http://localhost:3000/execute/${this.input.api}`,{
        method:  'POST',
        body: this.input.code
      })
        .then(function (response) {
            return response.text()
          })
        .then(data => {
          this.isSubmit = false;
          this.output.response = data
        })
        .catch(function (error) {
          this.isSubmit = false;
          console.log(`A http error occured: ${error}`);
        });
      }
    }
  }
}



</script>


<style scoped>
.show {
  visibility: visible;
}

.hide {
  visibility: hidden;
}

.submit {
  background-color: #04AA6D; /* Green */
  border: none;
  color: white;
  padding: 15px 32px;
  text-align: center;
  text-decoration: none;
  display: inline-block;
  font-size: 16px;
}
 

div {
  padding-right: 20px;
  padding-bottom: 20px;
  display:grid;
 }


#inputs {
  display: grid;
  width: 400px;
  height: fit-content;
  margin-right: 30px;
} 

input {
  margin-top: 5px;
  margin-bottom: 20px;
}


#form {
  display: flex;
  float: left;
}

.outputarea {
  border-width: 0;
}

.errors {
  color: red;
  font-size: larger;
}



#submitblock {
  display: flex;
  float: left;
  align-items: center;
}



</style>
