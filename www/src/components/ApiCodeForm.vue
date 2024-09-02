<template>
  <div id="form">
    <div>
      <div>
        <label>API name</label>
        <input v-model="input.api" type="text" placeholder="API name" name="api" required />   
      </div>
      <div>
          <label >Python code</label>
          <textarea v-model="input.code" type="text" placeholder="Python code here" name="code" rows="20" cols="100" required/>
      </div>
      <div>
          <div id="submitblock">
            <button class="submit" @click="submit">Submit</button>
            <img  v-if="isSubmit" src="@/assets/icons8-spinner.gif"/>
          </div>
          <label class="errors" v-if="errors.api">{{ errors.api }}</label>
          <label class="errors" v-if="errors.code">{{ errors.code }}</label>
      </div>
    </div>

    <div>    
      <div>
        <label >Output</label>
        <textarea v-model="output.response"  rows="30" cols="100" class="outputarea" />
      </div>
    </div>
  </div>
</template>


<script lang="ts">
declare interface FormErrors {
  api: string; 
  code: string;
}

declare interface AppResponse {
  response: string; 
}

export default {
  data() : {
    errors: FormErrors,
    input: FormErrors,
    output: AppResponse,
    isSubmit: boolean
  }{
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
      let isValid = (this.input.code == "" ) || (this.input.api == "" )
      if (this.input.code == "") {
        this.errors.code = "Python code required"
      } 
      if (this.input.api == "") {
        this.errors.api = "API name required"
      }
      return isValid
    },
    submit() {
      this.errors  = { api : "", code: ""};
      this.isSubmit = false;
      if (this.validate()) {
        return false
      } else {
      this.isSubmit = true;
      fetch(`http://localhost:3000/code/${this.input.api}`,{
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
        .catch( (error) => {
          this.isSubmit = false;
          console.log(`A http error occured: ${error}`);
          this.output.response = `A http error occured: ${error}`
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
